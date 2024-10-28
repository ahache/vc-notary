use axum::{
    routing::post, 
    Json, 
    Router, 
    response::IntoResponse,
    http::{HeaderValue, Method},
};
use tokio::net::TcpListener;
use serde::Deserialize;
use reqwest::Client;
use std::net::SocketAddr;
use dotenv::dotenv;
use std::env;
use tower_http::cors::CorsLayer;
use base64::encode;

#[derive(Deserialize)]
struct CodeData {
    code: String,
}

async fn fetch_bearer_token(Json(payload): Json<CodeData>) -> impl IntoResponse {
    let client_id = env::var("REDDIT_CLIENT_ID").expect("REDDIT_CLIENT_ID must be set");
    let client_secret = env::var("REDDIT_CLIENT_SECRET").expect("REDDIT_CLIENT_SECRET must be set");
    let redirect_uri = env::var("REDDIT_REDIRECT_URI").expect("REDDIT_REDIRECT_URI must be set");
    let client = Client::new();

    let credentials = format!("{}:{}", client_id, client_secret);
    let encoded_credentials = encode(credentials);

    let response = client
        .post("https://www.reddit.com/api/v1/access_token")
        .header("Authorization", format!("Basic {}", encoded_credentials))
        .header("User-Agent", "Notary3")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", &payload.code),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send()
        .await;

    match response {
        Ok(resp) => {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_else(|_| "Failed to read response body".to_string());
            println!("Reddit API response status: {}, body: {}", status, body);

            match serde_json::from_str::<serde_json::Value>(&body) {
                Ok(json) => Json(json).into_response(),
                Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse JSON").into_response(),
            }
        }
        Err(err) => {
            println!("Request to Reddit API failed: {:?}", err);
            (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "Request failed").into_response()
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = Router::new()
        .route("/api/request_vc", post(fetch_bearer_token))
        .layer(
            CorsLayer::new()
            .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::POST, Method::OPTIONS])
            .allow_headers(tower_http::cors::Any)
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}