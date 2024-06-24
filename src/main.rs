use binarite_cli::{commands::{context::CommandContext, temp::parse_command}, config::Config};
use cred_store::{CredStore, Credentials};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new();
    let mut credentials = Credentials::new()
        .set_file_name(".mg/auth".to_string())
        .build()
        .load()
        .expect("Failed to load credentials");

    let mut context = CommandContext {
        config: &config,
        cred_store: &mut credentials,
    };

    parse_command(&mut context).await;

    Ok(())
}
