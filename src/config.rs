include!(concat!(env!("OUT_DIR"), "/oauth.rs"));

#[derive(Debug)]
pub struct Config {
    pub domain: &'static str,
    pub client_id: &'static str,
    pub audience: &'static str,
    pub binarite_url: &'static str,
}

impl Config {
    pub fn new() -> Self {
        Self {
            domain: auth0_domain,
            client_id: auth0_client_id,
            audience: auth0_audience,
            binarite_url,
        }
    }
}