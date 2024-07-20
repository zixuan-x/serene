use axum::Router;
mod bff;

pub fn routes() -> Router {
    Router::new().nest("/bff", bff::routes())
}
