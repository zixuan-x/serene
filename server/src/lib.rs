use axum::{http::StatusCode, response::IntoResponse, Router};

mod routes;

pub fn router() -> Router {
    Router::new().merge(routes::routes()).fallback(handle_404)
}

async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}
