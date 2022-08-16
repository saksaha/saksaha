use super::WalletCredential;
use crate::WalletError;

pub struct CredentialManager {
    candidates: Vec<String>,
    credential: WalletCredential,
}

impl CredentialManager {
    pub fn init(
        wallet_credential: WalletCredential,
    ) -> Result<CredentialManager, WalletError> {
        let m = CredentialManager {
            candidates: vec![],
            credential: wallet_credential,
        };

        Ok(m)
    }

    #[inline]
    pub fn get_credential(&self) -> &WalletCredential {
        &self.credential
    }

    #[inline]
    pub fn get_acc_addr(&self) -> &String {
        &self.credential.acc_addr
    }

    pub fn get_candidates(&self) -> &Vec<String> {
        &self.candidates
    }
}
