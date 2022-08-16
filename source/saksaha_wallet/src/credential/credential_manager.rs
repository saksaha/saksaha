use super::WalletCredential;
use crate::WalletError;

pub struct CredentialManager {
    candidates: Vec<String>,
    curr_credential: WalletCredential,
}

impl CredentialManager {
    pub fn init(
        public_key: String,
        secret: String,
    ) -> Result<CredentialManager, WalletError> {
        let wallet_credential = WalletCredential::load(public_key, secret)?;

        let m = CredentialManager {
            candidates: vec![],
            curr_credential: wallet_credential,
        };

        Ok(m)
    }

    #[inline]
    pub fn get_curr_credential(&self) -> &WalletCredential {
        &self.curr_credential
    }

    pub fn get_candidates(&self) -> &Vec<String> {
        &self.candidates
    }
}
