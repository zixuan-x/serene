use axum::Router;

mod routes;

pub fn app() -> Router {
    Router::new().merge(routes::routes())
}
