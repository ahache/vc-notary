use reqwest::Client;
use std::env;
use base64::encode;

pub async fn fetch_bearer_token(code: String) -> Result<String, reqwest::Error> {
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
            ("code", &code),
            ("redirect_uri", redirect_uri.as_str()),
        ])
        .send()
        .await?;

    let text = response.text().await?;
    Ok(text)
}
