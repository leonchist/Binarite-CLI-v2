use cred_store::CredStore;

use super::context::CommandContext;

pub fn logout<T: CredStore>(context: &mut CommandContext<T>) {
    if context.cred_store.delete().is_err() {
        println!("No credentials found.");
        return;
    }

    println!("Logged out.");
}
