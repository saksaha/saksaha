use crate::{db::WalletDB, CredentialManager};

pub(crate) struct WalletApis {
    pub db: WalletDB,
    pub credential_manager: CredentialManager,
}
