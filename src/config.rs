use dotenvy::dotenv;
use std::env;

#[derive(Debug)]
pub struct Config {
    pub domain: String,
    pub client_id: String,
    pub audience: String,
    pub binarite_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        dotenv().ok();
        let domain = env::var("AUTH0_DOMAIN")?;
        let client_id = env::var("AUTH0_CLIENT_ID")?;
        let audience = env::var("AUTH0_AUDIENCE")?;
        let binarite_url = env::var("BINARITE_URL")?;

        Ok(Self {
            domain,
            client_id,
            audience,
            binarite_url,
        })
    }
}