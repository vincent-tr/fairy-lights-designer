use anyhow::{Context, Result};
use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use futures_util::TryStreamExt;
use mongodb::{bson::doc, options::ClientOptions, results::DeleteResult, Client, Collection};
use serde::{Deserialize, Serialize};
use url::Url;

use super::WebError;

// defining Program type
#[derive(Debug, Deserialize, Serialize)]
struct Program {
    #[serde(rename = "_id")]
    id: String,
    name: String,
    content: String,
}

const COLLECTION_NAME: &str = "programs";

pub async fn build(url: &str) -> Result<Router> {
    let db_name = {
        let url = Url::parse(url).context("Invalid MongoDB URL")?;
        url.path().trim_start_matches('/').to_string()
    };

    let mut options = ClientOptions::parse(url).await.context("Failed to parse MongoDB URL")?;
    options.direct_connection = Some(true);
    let client = Client::with_options(options).context("Failed to connect to MongoDB")?;
    let collection: Collection<Program> = client.database(&db_name).collection(COLLECTION_NAME);

    Ok(Router::new()
        .route("/api/create", post(create_program))
        .route("/api/read/{id}", get(read_program))
        .route("/api/update", put(update_program))
        .route("/api/delete/{id}", delete(delete_program))
        .route("/api/list", get(list_programs))
        .with_state(collection))
}

async fn create_program(
    State(db): State<Collection<Program>>,
    Json(input): Json<Program>,
) -> Result<Json<String>, WebError> {
    let result = db.insert_one(input).await?;

    let id = result
        .inserted_id
        .as_str()
        .context("Failed to get inserted ID")?
        .to_string();

    Ok(Json(id))
}

async fn read_program(
    State(db): State<Collection<Program>>,
    Path(id): Path<String>,
) -> Result<Json<Program>, WebError> {
    let program = db
        .find_one(doc! { "_id": id })
        .await?
        .context("Program not found")?;

    Ok(Json(program))
}

async fn update_program(
    State(db): State<Collection<Program>>,
    Json(input): Json<Program>,
) -> Result<Json<()>, WebError> {
    let result = db
        .replace_one(doc! { "_id": input.id.clone() }, input)
        .await?;

    if result.modified_count == 0 {
        None.context("Program not found")?;
    }

    Ok(Json(()))
}

async fn delete_program(
    State(db): State<Collection<Program>>,
    Path(id): Path<String>,
) -> Result<Json<DeleteResult>, WebError> {
    let result = db.delete_one(doc! { "_id": id }).await?;

    if result.deleted_count == 0 {
        None.context("Program not found")?;
    }

    Ok(Json(result))
}

async fn list_programs(
    State(db): State<Collection<Program>>,
) -> Result<Json<Vec<Program>>, WebError> {
    let cursor = db
        .find(doc! {})
        .sort(doc! { "name": 1 })
        .projection(doc! { "content": 0 })
        .await?;

    let programs = cursor.try_collect().await?;

    Ok(Json(programs))
}
