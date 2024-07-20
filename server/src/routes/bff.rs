use axum::{routing::get, Router};

pub fn routes() -> Router {
    Router::new()
        .route("/check", get(check))
        .route("/login", get(login))
        // TODO: Should this be get or post?
        .route("/logout", get(logout))
}

async fn check() {}
async fn login() {}
async fn logout() {}
