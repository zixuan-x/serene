use axum::{http::StatusCode, response::IntoResponse, Router};
mod bff;

pub fn routes() -> Router {
    Router::new()
        .nest("/bff", bff::routes())
        .fallback(handle_404)
}

async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}
