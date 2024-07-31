use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;

pub fn router() -> Router {
    tracing_subscriber::registry()
        // .with(
        //     tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        //         "example_tracing_aka_logging=debug,tower_http=debug,axum::rejection=trace".into()
        //     }),
        // )
        .with(tracing_subscriber::fmt::layer())
        .init();
    Router::new().nest("/api/v0", routes::routes()).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .into_inner(),
    )
}
