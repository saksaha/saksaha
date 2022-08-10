use super::WalletApis;
use crate::{
    types::Status, wallet::apis::decode_hex_string_to_u64, WalletError,
};
use sak_crypto::{Bls12, Proof};
use sak_proofs::{NewCoin, OldCoin};
use sak_types::Balance;
use saksaha::{generate_proof_1_to_2, get_auth_path};

impl WalletApis {
    pub async fn get_balance(
        &self,
        id: String,
        _key: String,
    ) -> Result<Balance, WalletError> {
        println!("wallet apis, get_balance, id: {}", id);

        let latest_cm_idx = match self.db.schema.get_latest_cm_idx()? {
            Some(i) => i,
            None => {
                return Err(format!(
                    "Wallet is empty, the balance must be zero"
                )
                .into())
            }
        };

        let mut balance: u64 = 0;

        // debug!("latest cm idx: {:?}", latest_cm_idx);

        for cm_idx in 0..=latest_cm_idx {
            let cm: String = match self.db.schema.get_cm(&cm_idx).await {
                Ok(c) => match c {
                    Some(c) => c,
                    None => {
                        return Err(format!(
                            "No cm has been found at idx: {:?}",
                            cm_idx
                        )
                        .into())
                    }
                },
                Err(err) => {
                    return Err(
                        format!("Failed to get cm, err: {:?}", err).into()
                    )
                }
            };

            let user = match self.db.schema.get_user_id(&cm).await? {
                Some(u) => u,
                None => return Err(format!("Failed to get user_id").into()),
            };

            let _status = match self.db.schema.get_status(&cm).await? {
                Some(s) => {
                    if s == "Used" {
                        continue;
                    }
                }
                None => return Err(format!("Failed to get status").into()),
            };

            if user == id {
                let v = match self.db.schema.get_v(&cm).await? {
                    Some(v) => {
                        let v = decode_hex_string_to_u64(&v).await?;

                        v
                    }
                    None => return Err(format!("Failed to get value").into()),
                };

                balance += v;
            }
        }

        let b = Balance { val: balance };

        Ok(b)
    }

    pub async fn send_tx(
        &self,
        val: usize,
        // send_tx_req: SendTxRequest,
    ) -> Result<(), WalletError> {
        {
            // let val = send_tx_req.get_value();

            // let old_coin = prepare_old_coin(val).await?;
            // saksaha::send_tx_pour(ctr_addr, req_type, arg).await?;
        }

        // from db
        // check if some data is something
        // return
        Ok(())
    }

    pub(crate) async fn prepare_old_coin(
        &self,
        idx: [u8; 32],
    ) -> Result<OldCoin, WalletError> {
        //TODO :
        let idx = 0;
        let auth_path = get_auth_path(idx).await?;
        let coin = OldCoin::default();

        self.put_old_coin(&coin);

        Ok(coin)
    }

    pub(crate) async fn put_old_coin(&self, old_coin: &OldCoin) {
        // self.db.schema.put_old_coin
    }

    pub(crate) async fn get_old_coin(&self, cm: String) -> OldCoin {
        // self.db.schema.get_old_coin
        OldCoin::default()
    }

    pub(crate) async fn put_new_coin(&self, new_coin: NewCoin) {
        // self.db.schema.put_old_coin
    }

    pub(crate) async fn generate_new_coin(&self, value: u64) -> NewCoin {
        NewCoin::default()
    }

    pub(crate) async fn generate_proof(
        &self,
        old_coin: OldCoin,
        new_coin_1: NewCoin,
        new_coin_2: NewCoin,
    ) -> Result<Proof<Bls12>, WalletError> {
        let pi =
            generate_proof_1_to_2(old_coin, new_coin_1, new_coin_2).await?;

        Ok(pi)
    }

    // rpc routes features
    // saksaha::send_tx_pour(ctr_addr, req_type, arg)
    // saksaha::send_tx_mint(ctr_addr, req_type, arg)
}
