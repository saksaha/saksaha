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
            candidates: vec![wallet_credential.acc_addr.clone()],
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

    pub fn put_candidates(&mut self, candidate: String) {
        let candidates_list = &mut self.candidates;

        candidates_list.push(candidate);
    }
}
