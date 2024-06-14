use crate::commands::CommandContext;
use base64::Engine;
use cred_store::CredStore;
use oauth2::{
    basic::BasicClient, reqwest::http_client, AuthUrl, ClientId, DeviceAuthorizationUrl,
    RefreshToken, TokenResponse, TokenUrl,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Claims {
    exp: i64,
}

fn decode_claims_without_verification(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    let parts: Vec<&str> = token.split('.').collect();

    if parts.len() != 3 {
        return Err("Token format is incorrect".into());
    }

    let payload = parts[1];
    let decoded_payload = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(payload)?;
    let claims: Claims = serde_json::from_slice(&decoded_payload)?;

    Ok(claims)
}

fn is_token_expired(token: &str) -> bool {
    let claims = match decode_claims_without_verification(token) {
        Ok(claims) => claims,
        Err(_) => return true,
    };

    let now = chrono::Utc::now().timestamp();

    claims.exp < now
}

pub fn refresh_access_token(
    domain: &str,
    client_id: &str,
    refresh_token: &str,
) -> Result<super::token_response::TokenResponse, Box<dyn std::error::Error>> {
    let device_auth_url =
        DeviceAuthorizationUrl::new(format!("https://{}/oauth/device/code", domain))?;
    let client = BasicClient::new(
        ClientId::new(client_id.to_string()),
        None,
        AuthUrl::new(format!("https://{}/authorize", domain))?,
        Some(TokenUrl::new(format!("https://{}/oauth/token", domain))?),
    )
    .set_device_authorization_url(device_auth_url);

    let token_result = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token.to_string()))
        .request(http_client);

    match token_result {
        Ok(result) => {
            let access_token = result.access_token().secret().clone();
            let refresh_token = if result.refresh_token().is_some() {
                result.refresh_token().unwrap().secret().clone()
            } else {
                String::new()
            };
            let expires_in = result.expires_in().unwrap().as_secs();
            // scopes not extracted.
            let scopes = result
                .scopes()
                .unwrap()
                .iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
                .join(", ");

            Ok(super::token_response::TokenResponse {
                access_token: Some(access_token),
                token_type: None,
                refresh_token: Some(refresh_token),
                expires_in: Some(expires_in as usize),
                scope: Some(scopes),
            })
        }
        Err(e) => Err(Box::new(e)),
    }
}

pub fn get_token<T: CredStore>(
    context: &mut CommandContext<T>,
) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut credentials = context.cred_store.load()?;
    let access_token = credentials.get("access_token").cloned();
    let refresh_token = credentials.get("refresh_token").cloned();

    match (access_token, refresh_token) {
        (Some(at), Some(rt)) => {
            if is_token_expired(&at) {
                let token_response =
                    refresh_access_token(context.config.domain, context.config.client_id, &rt)?;
                let new_access_token = token_response.access_token.unwrap();
                let new_refresh_token = token_response.refresh_token.unwrap();

                credentials
                    .add("access_token".to_string(), new_access_token.clone())
                    .add("refresh_token".to_string(), new_refresh_token);

                credentials.save()?;

                Ok(Some(new_access_token))
            } else {
                Ok(Some(at))
            }
        }
        _ => Ok(None),
    }
}
