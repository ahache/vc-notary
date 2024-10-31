use axum::{
    extract::Multipart,
    routing::post,
    Router,
    // Json, 
    response::IntoResponse,
    // http::{HeaderValue, Method},
};
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tlsn_core::{
    presentation::{Presentation, PresentationOutput},
    CryptoProvider,
};
use serde::Deserialize;

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
    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await.unwrap();

        let presentation: Presentation = bincode::deserialize(&data).unwrap();

        let provider = CryptoProvider::default();

        let PresentationOutput { transcript, server_name, .. } = presentation.verify(&provider).unwrap();

        println!("Server name: {}", server_name.unwrap());

        let transcript = transcript.unwrap();

        let bytes = transcript.received_unsafe();
        let response_text = String::from_utf8_lossy(bytes);
        let parts: Vec<&str> = response_text.split("\r\n\r\n").collect();
        let body = parts[1].as_bytes();
        let parsed_body: RedditResponse = serde_json::from_slice(body).expect("Failed to parse body");

        // Grabs the first subreddit from the response
        let subreddit_url = &parsed_body.data.children[0].data.url;
        println!("Subreddit URL: {}", subreddit_url);

    }

    (
        axum::http::StatusCode::OK,
        "Processing request, please check back later.",
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
