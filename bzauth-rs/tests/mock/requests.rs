use super::*;

pub async fn make_authorisation_request() -> reqwest::Response {
    // Fetch the authorization URL
    let client = server_provider::get_client();
    let (url, _) = client
        .authorize_url(oauth2::CsrfToken::new_random)
        .add_scope(oauth2::Scope::new("read".to_string()))
        .url();
    println!("Authorization URL: {}", url);

    // Make the reqwest
    let response = reqwest::get(url.to_string())
        .await
        .expect("Failed to make request to auth server");

    response
}

pub async fn make_callback_request(auth_url: &str, provider_name: &str) -> reqwest::Response {
    let client = reqwest::Client::new();
    let response = client
        .post(format!("{}/callback/{}", auth_url, provider_name))
        .query(&[("code", "mock_auth_code"), ("state", "mock_state")])
        .send()
        .await
        .expect("Failed to make request to auth server");

    response
}
