use cred_store::CredStore;

use crate::config::Config;

#[derive(Debug)]
pub struct CommandContext<'a, T: CredStore> {
    pub config: &'a Config,
    pub cred_store: &'a mut T,
}
