use anyhow::{Context, Result};
use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use bson::Bson;
use futures_util::TryStreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
    Client, Collection,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{debug, info};
use url::Url;

use super::WebError;

#[derive(Debug, Deserialize, Serialize)]
struct Program {
    #[serde(rename = "_id")]
    id: ObjectId,
    name: String,
    content: Bson,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProgramModel {
    name: String,
    content: Value,
}

#[derive(Debug, Deserialize, Serialize)]
struct ListItemModel {
    id: String,
    name: String,
}

type ListModel = Vec<ListItemModel>;

const COLLECTION_NAME: &str = "programs";

pub async fn build(url: &str) -> Result<Router> {
    let db_name = {
        let url = Url::parse(url).context("Invalid MongoDB URL")?;
        url.path().trim_start_matches('/').to_string()
    };

    let mut options = ClientOptions::parse(url)
        .await
        .context("Failed to parse MongoDB URL")?;
    options.direct_connection = Some(true);
    let client = Client::with_options(options).context("Failed to connect to MongoDB")?;
    let collection: Collection<Program> = client.database(&db_name).collection(COLLECTION_NAME);

    Ok(Router::new()
        .route("/api/create", post(create_program))
        .route("/api/read/{id}", get(read_program))
        .route("/api/update/{id}", put(update_program))
        .route("/api/delete/{id}", delete(delete_program))
        .route("/api/list", get(list_programs))
        .with_state(collection))
}

async fn create_program(
    State(db): State<Collection<Program>>,
    Json(input): Json<ProgramModel>,
) -> Result<Json<String>, WebError> {
    let oid = ObjectId::new();
    let id = oid.to_hex();
    debug!("Create program: {}", id);

    let program = Program {
        id: oid,
        name: input.name,
        content: bson::to_bson(&input.content).context("Failed to serialize content")?,
    };

    let result = db.insert_one(program).await?;

    if result.inserted_id != bson::Bson::ObjectId(oid) {
        None.context("Failed to insert program")?;
    }

    info!("Created program: {}", id);
    Ok(Json(id))
}

async fn read_program(
    State(db): State<Collection<Program>>,
    Path(id): Path<String>,
) -> Result<Json<ProgramModel>, WebError> {
    debug!("Read program: {}", id);
    let oid = ObjectId::parse_str(&id).context("Invalid ID")?;

    let program = db
        .find_one(doc! { "_id": &oid })
        .await?
        .context("Program not found")?;

    let model = ProgramModel {
        name: program.name,
        content: program.content.into(),
    };

    Ok(Json(model))
}

async fn update_program(
    State(db): State<Collection<Program>>,
    Path(id): Path<String>,
    Json(input): Json<ProgramModel>,
) -> Result<Json<()>, WebError> {
    debug!("Update program: {}", id);
    let oid = ObjectId::parse_str(&id).context("Invalid ID")?;

    let program = Program {
        id: oid,
        name: input.name,
        content: bson::to_bson(&input.content).context("Failed to serialize content")?,
    };

    let result = db.replace_one(doc! { "_id": &oid }, program).await?;

    if result.matched_count == 0 {
        None.context("Program not found")?;
    }

    if result.modified_count == 1 {
        info!("Updated program: {}", id);
    } else {
        debug!("No change to program: {}", id);
    }

    Ok(Json(()))
}

async fn delete_program(
    State(db): State<Collection<Program>>,
    Path(id): Path<String>,
) -> Result<Json<()>, WebError> {
    debug!("Delete program: {}", id);
    let oid = ObjectId::parse_str(&id).context("Invalid ID")?;

    let result = db.delete_one(doc! { "_id": &oid }).await?;

    if result.deleted_count == 0 {
        None.context("Program not found")?;
    }

    info!("Deleted program: {}", id);
    Ok(Json(()))
}

async fn list_programs(State(db): State<Collection<Program>>) -> Result<Json<ListModel>, WebError> {
    debug!("List programs");

    let cursor = db.find(doc! {}).sort(doc! { "name": 1 }).await?;
    // TODO: avoid to fetch content

    let programs: Vec<Program> = cursor.try_collect().await?;

    let model: ListModel = programs
        .into_iter()
        .map(|program| ListItemModel {
            id: program.id.to_hex(),
            name: program.name,
        })
        .collect();

    Ok(Json(model))
}
