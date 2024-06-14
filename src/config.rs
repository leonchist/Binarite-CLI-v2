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
            domain: AUTH0_DOMAIN,
            client_id: AUTH0_CLIENT_ID,
            audience: AUTH0_AUDIENCE,
            binarite_url: BINARITE_URL,
        }
    }
}
