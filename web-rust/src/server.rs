use dioxus::prelude::{server, server_fn, ServerFnError};

#[server(PostServerData)]
pub(crate) async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    println!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
pub(crate) async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}
