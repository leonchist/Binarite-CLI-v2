use cred_store::CredStore;
use crate::auth::get_token::get_token;

use super::CommandContext;

pub fn refresh<T: CredStore>(context: &mut CommandContext<T>) {
    let access_token = match get_token(context) {
        Ok(token) => match token {
            Some(token) => token,
            None => {
                eprintln!("You must login first.");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("Couldn't get credentials: {}.  Try to login again.", e);
            std::process::exit(1);
        }
    };
}