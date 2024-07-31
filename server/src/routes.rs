use axum::{http::StatusCode, response::IntoResponse, Router};
mod auth;

pub fn routes() -> Router {
    Router::new()
        .nest("/auth", auth::routes())
        .fallback(handle_404)
}

async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}
