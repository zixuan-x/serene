use axum::Router;

mod routes;

pub fn app() -> Router {
    let app = Router::new().merge(routes::routes());
    app
}
