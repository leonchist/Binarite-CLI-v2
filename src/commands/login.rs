use cred_store::CredStore;

use crate::auth::login;

use super::context::CommandContext;

fn save_tokens<T: CredStore>(
    access_token: &str,
    refresh_token: &str,
    context: &mut CommandContext<T>,
) -> Result<(), std::io::Error> {
    context
        .cred_store
        .clear()
        .add("access_token".to_string(), access_token.to_string())
        .add("refresh_token".to_string(), refresh_token.to_string())
        .save()
}

pub async fn login<'a, T: CredStore>(context: &mut CommandContext<'a, T>) {
    match login::login(context.config).await {
        Ok(resp) => {
            let access_token = resp.access_token.clone().unwrap();
            let refresh_token = resp.refresh_token.clone().unwrap_or_default();
            println!();
            println!("Access Token: {}", access_token);
            if save_tokens(&access_token, &refresh_token, context).is_err() {
                eprintln!("Couldn't configure credentials.");
                std::process::exit(1);
            }
        }
        Err(e) => println!("Error logging in: {}", e),
    }
}
