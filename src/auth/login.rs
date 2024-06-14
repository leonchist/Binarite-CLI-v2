use oauth2::{basic::BasicClient, reqwest::async_http_client, AuthUrl, ClientId, DeviceAuthorizationUrl, Scope, StandardDeviceAuthorizationResponse, TokenResponse, TokenUrl};
use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};

use crate::config::Config;

#[derive(Serialize, Deserialize)]
struct DeviceAuthResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    verification_uri_complete: String,
    expires_in: usize,
    interval: usize,
}

pub async fn login(config: &Config) -> Result<super::token_response::TokenResponse, Box<dyn std::error::Error>> {

    let device_auth_url = DeviceAuthorizationUrl::new(format!("https://{}/oauth/device/code", config.domain))?;
    let client =
    BasicClient::new(
        ClientId::new(config.client_id.to_string()),
        None,
        AuthUrl::new(format!("https://{}/authorize", config.domain))?,
        Some(TokenUrl::new(format!("https://{}/oauth/token", config.domain))?),
    )
    .set_device_authorization_url(device_auth_url);

    let details: StandardDeviceAuthorizationResponse = client
        .exchange_device_code()?
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .add_scope(Scope::new("offline_access".to_string()))
        .request_async(async_http_client)
        .await?;

    println!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        **details.verification_uri(),
        details.user_code().secret()
    );

    let complete_uri = details.verification_uri_complete().unwrap();

    _ = open::that(complete_uri.secret());

    let mut sp = Spinner::new(Spinners::Dots9, "Polling for token".into());

    let token_result = client
        .exchange_device_access_token(&details)
        .request_async(async_http_client, tokio::time::sleep, None)
        .await?;
    
     sp.stop();


    let access_token = token_result.access_token().secret().clone();
    let refresh_token = if token_result.refresh_token().is_some() {
        token_result.refresh_token().unwrap().secret().clone()
    }
    else {
        String::new()
    };
    let expires_in = token_result.expires_in().unwrap().as_secs();
    let scopes = token_result.scopes().unwrap().iter().map(| s | s.to_string()).collect::<Vec<String>>().join(", ");

   Ok(super::token_response::TokenResponse { access_token: Some(access_token), token_type: None, refresh_token: Some(refresh_token), expires_in: Some(expires_in as usize), scope: Some(scopes) })
}