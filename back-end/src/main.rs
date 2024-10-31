mod auth;
mod notarize;
mod present;
mod verify;

use auth::fetch_bearer_token;
use notarize::notarize_api_data;
use present::build_presentation;
use verify::verify_presentation;

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

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

async fn process_user_and_vc(Json(payload): Json<CodeData>) -> impl IntoResponse {
    // let response_body = fetch_bearer_token(payload.code).await.unwrap();
    // let token_response: TokenResponse = serde_json::from_str(&response_body).unwrap();

    // let access_token = token_response.access_token;

    // // Communicate with the notary server and get attestation and secrets
    // notarize_api_data(access_token).await;

    // // Build the presentation
    // build_presentation();

    // // Verify the presentation
    // verify_presentation();

    let client = Client::new();
    let file_path = "vcnotary.presentation.tlsn";
    let form = reqwest::multipart::Form::new()
        .file("file", file_path)
        .await
        .expect("Failed to create form file");

    let response = client
        .post("http://localhost:3333/request_vc")
        .multipart(form)
        .send()
        .await
        .expect("Failed to send request");

    println!("Response: {:?}", response);

    (
        axum::http::StatusCode::OK,
        "Processing request, please check back later.",
    )
        .into_response()
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let app = Router::new()
        .route("/api/request_vc", post(process_user_and_vc))
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
