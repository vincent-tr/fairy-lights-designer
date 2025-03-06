use anyhow::{Context, Result};
use axum::{
    body::Bytes,
    extract::Path,
    http::header,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use http_body_util::Full;
use include_dir::{include_dir, Dir};

use super::WebError;

static UI_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../ui/dist");

pub fn build() -> Router{
    Router::new()
        .route("/", get(serve_index))
        .route("/{*path}", get(serve_static_file))
}

async fn serve_index() -> Result<impl IntoResponse, WebError> {
    serve_asset("index.html").await
}

async fn serve_static_file(path: Path<String>) -> Result<impl IntoResponse, WebError> {
    serve_asset(&path.0).await
}

async fn serve_asset(path: &str) -> Result<impl IntoResponse, WebError> {
    let file = UI_DIR
        .get_file(path)
        .context(format!("Missing asset '{}'", path))?;

    let mime_type = mime_guess::from_path(path)
        .first_or_octet_stream()
        .to_string();

    let response = Response::builder()
        .header(header::CONTENT_TYPE, mime_type)
        .body(Full::new(Bytes::from_static(file.contents())))
        .context("Failed to build response")?;

    Ok(response)
}
