use axum::{
    extract::Multipart,
    routing::post,
    Router,
    Json, 
    response::IntoResponse,
};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tlsn_core::{
    presentation::{Presentation, PresentationOutput},
    CryptoProvider,
};
use serde::Deserialize;
use reqwest::Client;

#[derive(Debug, Deserialize)]
struct RedditResponse {
    data: ListingData,
}

#[derive(Debug, Deserialize)]
struct ListingData {
    children: Vec<Child>,
}

#[derive(Debug, Deserialize)]
struct Child {
    data: SubredditData,
}

#[derive(Debug, Deserialize)]
struct SubredditData {
    url: String,
}

async fn request_vc(mut multipart: Multipart) -> impl IntoResponse {
    let mut did = String::new();
    let mut subreddit_url = String::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "file" => {
                let data = field.bytes().await.unwrap();
                let presentation: Presentation = bincode::deserialize(&data).unwrap();

                let provider = CryptoProvider::default();

                let PresentationOutput { transcript, .. } = presentation.verify(&provider).unwrap();

                let transcript = transcript.unwrap();
                let bytes = transcript.received_unsafe();
                let response_text = String::from_utf8_lossy(bytes);
                let parts: Vec<&str> = response_text.split("\r\n\r\n").collect();
                let body = parts[1].as_bytes();

                let parsed_body: RedditResponse = serde_json::from_slice(body).expect("Failed to parse body");
                subreddit_url = parsed_body.data.children[0].data.url.clone();
            }
            "did" => {
                did = String::from_utf8(field.bytes().await.unwrap().to_vec()).unwrap();
            },
            _ => {}
        }
    }

    let client = Client::new();
    let response = client.post("http://localhost:3334/create-credential")
        .json(&serde_json::json!({
            "did": did,
            "subreddit": subreddit_url
        }))
        .send().await.unwrap();

    let credential = response.json::<serde_json::Value>().await.unwrap();
    
    (
        axum::http::StatusCode::OK,
        Json(credential),
    )
        .into_response()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/request_vc", post(request_vc));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    println!("Server running on http://{}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
