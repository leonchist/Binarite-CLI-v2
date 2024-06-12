use crate::config::Config;
use oauth2::{basic::BasicClient, reqwest::http_client, AuthUrl, ClientId, DeviceAuthorizationUrl, Scope, StandardDeviceAuthorizationResponse, TokenResponse, TokenUrl};
use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};

#[derive(Serialize, Deserialize)]
struct DeviceAuthResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    verification_uri_complete: String,
    expires_in: usize,
    interval: usize,
}

pub fn login(config: &Config) -> Result<super::token_response::TokenResponse, Box<dyn std::error::Error>> {

    let device_auth_url = DeviceAuthorizationUrl::new(format!("https://{}/oauth/device/code", config.domain))?;
    let client =
    BasicClient::new(
        ClientId::new(config.client_id.clone()),
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
    .request(http_client)?;

    dbg!(&details);

    dbg!(&client);

    

    println!(
        "Open this URL in your browser:\n{}\nand enter the code: {}",
        details.verification_uri().to_string(),
        details.user_code().secret().to_string()
    );

    let complete_uri = details.verification_uri_complete().unwrap();

    _ = open::that(complete_uri.secret());

    // let start_instant = Instant::now();
    // let expiry_duration = Duration::from_secs(device_auth_response.expires_in as u64);

    let mut sp = Spinner::new(Spinners::Dots9, "Polling for token".into());
    
    let token_result =
        client
        .exchange_device_access_token(&details)
        .request(http_client, std::thread::sleep, None)?;

    sp.stop();

    dbg!(&token_result);

    let access_token = token_result.access_token().secret().clone();
    let token_type = token_result.token_type();
    let refresh_token = if token_result.refresh_token().is_some() {
        token_result.refresh_token().unwrap().secret().clone()
    }
    else {
        String::new()
    };
    let expires_in = token_result.expires_in().unwrap().as_secs();
    // scopes not extracted.
    let scopes = token_result.scopes().unwrap().iter().map(| s | s.to_string()).collect::<Vec<String>>().join(", ");



    Ok(super::token_response::TokenResponse { access_token: Some(access_token), token_type: None, refresh_token: Some(refresh_token), expires_in: Some(expires_in as usize), scope: Some(scopes) })
    // let client = Client::new();
    // let resp = client
    //     .post(&format!("https://{}/oauth/device/code", config.domain))
    //     .form(&[
    //         ("client_id", config.client_id.as_str()),
    //         ("audience", config.audience.as_str()),
    //         ("scope", "openid profile email offline_access"),
    //     ])
    //     .send();

    // let response = match resp {
    //     Ok(resp) => resp,
    //     Err(e) => return Err(e.into()),
    // };
    // let device_auth_response: DeviceAuthResponse = match response.json::<DeviceAuthResponse>() {
    //     Ok(resp) => resp,
    //     Err(e) => return Err(e.into()),
    // };

    // println!(
    //     "Go to {} and enter the code: {}",
    //     device_auth_response.verification_uri, device_auth_response.user_code
    // );

    // _ = open::that(device_auth_response.verification_uri_complete);

    // let token_endpoint = format!("https://{}/oauth/token", config.domain);

    // let start_instant = Instant::now();
    // let expiry_duration = Duration::from_secs(device_auth_response.expires_in as u64);

    // let mut sp = Spinner::new(Spinners::Dots9, "Polling for token".into());

    // loop {
    //     if Instant::now() >= start_instant + expiry_duration {
    //         sp.stop();
    //         return Err(Box::new(std::io::Error::new(
    //             std::io::ErrorKind::TimedOut,
    //             "Device code has expired",
    //         )));
    //     }

    //     let resp_result = client
    //         .post(&token_endpoint)
    //         .form(&[
    //             ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
    //             ("device_code", &device_auth_response.device_code),
    //             ("client_id", config.client_id.as_str()),
    //         ])
    //         .send()
    //         .and_then(|res| res.json::<TokenResponse>());

    //     match resp_result {
    //         Ok(resp) => {
    //             if resp.access_token.is_some() {
    //                 sp.stop();
    //                 return Ok(resp);
    //             }
    //         }
    //         Err(e) => {
    //             sp.stop();
    //             return Err(Box::new(e));
    //         }
    //     }

    //     std::thread::sleep(std::time::Duration::from_secs(
    //         device_auth_response.interval as u64,
    //     ));
    //}
}