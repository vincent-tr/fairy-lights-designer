use axum::{http::StatusCode, response::{IntoResponse, Response}};


pub struct WebError(anyhow::Error);

impl IntoResponse for WebError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self.0.to_string()).into_response()
    }
}

impl<E> From<E> for WebError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
