use std::{collections::HashMap, env};

use axum::{
    extract::Query,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub fn routes() -> Router {
    Router::new()
        .route("/check", get(check))
        .route("/login", get(login))
        .route("/authorize", get(authorize))
        .route("/logout", get(logout))
}

#[derive(Serialize, Deserialize)]
struct OidcConfig {
    authorization_endpoint: String,
    token_endpoint: String,
    userinfo_endpoint: String,
}

async fn check() {}
async fn login() -> impl IntoResponse {
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID is not set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI is not set");
    let discovery_url = env::var("DISCOVERY_URL").expect("DISCOVERY_URL is not set");

    let oidc_config = reqwest::get(&discovery_url)
        .await
        .unwrap()
        .json::<OidcConfig>()
        .await
        .unwrap();

    let authorization_url = format!(
        "{}?client_id={}&redirect_uri={}&response_type=code&scope=openid",
        oidc_config.authorization_endpoint, client_id, redirect_uri,
    );
    Redirect::temporary(&authorization_url)
}
async fn authorize(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let code = params.get("code").expect("code is not set");
    let client_id = env::var("CLIENT_ID").expect("CLIENT_ID is not set");
    let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET is not set");
    let redirect_uri = env::var("REDIRECT_URI").expect("REDIRECT_URI is not set");
    let token_endpoint = env::var("TOKEN_ENDPOINT").expect("TOKEN_ENDPOINT is not set");

    let client = reqwest::Client::new();
    let response = client
        .post(&token_endpoint)
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("redirect_uri", &redirect_uri),
        ])
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp
                .text()
                .await
                .unwrap_or_else(|_| "Failed to read response body".to_string());
            println!("Response Status: {}", status);
            println!("Response Body: {}", body);

            if status.is_success() {
                // Handle successful response
                // Parse the response body if needed
                let json: Value = serde_json::from_str(&body)
                    .unwrap_or_else(|_| serde_json::json!({ "error": "Invalid JSON" }));
                println!("Parsed JSON: {:?}", json);
            } else {
                // Handle error response
                println!("Error: Received non-success status code");
            }
        }
        Err(e) => {
            println!("Error: Failed to send request - {}", e);
        }
    }

    // Return an appropriate response to the client
    "Authorization complete"
}
async fn logout() {}
