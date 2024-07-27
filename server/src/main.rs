use server::router;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router()).await.unwrap();
    Ok(())
}
