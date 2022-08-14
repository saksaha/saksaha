use crate::{credential::WalletCredential, db::WalletDB, CredentialManager};

pub(crate) struct WalletApis {
    pub db: WalletDB,
    // pub credential: WalletCredential,
    pub credential_manager: CredentialManager,
}
