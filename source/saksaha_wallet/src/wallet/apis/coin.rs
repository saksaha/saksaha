use super::WalletApis;

use crate::rpc::routes::v0::WalletSendTxRequest;
use crate::types::Coin;
use crate::{wallet::apis::decode_hex_string_to_u64, WalletError};
use log::debug;
use sak_crypto::{
    groth16, mimc, os_rng, Bls12, Circuit, Hasher, Proof, Scalar, ScalarExt,
};
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};
use sak_types::Balance;
use saksaha::{generate_proof_1_to_2, get_auth_path};
pub const GAS: u64 = 10;

impl WalletApis {
    pub async fn get_balance(
        &self,
        id: String,
        key: String,
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
            let cm: String = match self.db.schema.get_cm(&cm_idx) {
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

            let user = match self.db.schema.get_user_id(&cm)? {
                Some(u) => u,
                None => return Err(format!("Failed to get user_id").into()),
            };

            if user == id {
                let v = match self.db.schema.get_v(&cm)? {
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
        req: WalletSendTxRequest,
        // send_tx_req: SendTxRequest,
    ) -> Result<(), WalletError> {
        let WalletSendTxRequest {
            gas,
            ctr_addr,
            req_type,
            args,
        } = req;

        let id = String::from("user_1");
        let key = String::from("user1pw");

        // Check balance
        {
            let my_balance = self.get_balance(id, key).await?;
            let check_enough_balalnce = my_balance.val > GAS;
            if !check_enough_balalnce {
                return Err(format!("You don't have enough coin").into());
            }
        }

        // Get the old coin
        let cm_idx = match self.db.schema.get_latest_cm_idx()? {
            Some(i) => i,
            None => {
                return Err(format!(
                    "Wallet is empty, the balance must be zero"
                )
                .into())
            }
        };

        let auth_path = {
            let response = saksaha::get_auth_path(cm_idx).await?;

            let result = response.result.ok_or(format!("error"))?;

            result.auth_path
        };

        let old_coin = self.get_old_coin(cm_idx, auth_path).await?;

        let old_coin_v = match old_coin.v {
            Some(v) => 0,
            None => 0,
        };

        // Generate two new coins
        let new_coin_1 = Coin::new(old_coin_v - GAS, id);
        let new_coin_2 = Coin::new(0, id);

        // Generate Pi
        let pi = generate_proof_1_to_2(old_coin, new_1, new_2).await?;

        // send tx pour (pi, sn_1, sn_2, cm_1, cm_2, merkle_rt)

        saksaha::send_tx_pour(ctr_addr, req_type, args).await?;

        // update the state to "used"

        // from db
        // check if some data is something
        // return
        Ok(())
    }

    pub(crate) async fn get_old_coin(
        &self,
        cm_idx: u128,
        auth_path: Vec<([u8; 32], bool)>,
    ) -> Result<OldCoin, WalletError> {
        let cm: String = match self.db.schema.get_cm(&cm_idx) {
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
                return Err(format!("Failed to get cm, err: {:?}", err).into())
            }
        };

        let mut old_coin = self.db.schema.get_coin(&cm)?;
        let auth_path_vec = vec![];
        // let auth_path_arr = [Option<(Scalar, bool)>; CM_TREE_DEPTH as usize];
        for (arr, dir) in auth_path {
            let scalar = ScalarExt::parse_vec(arr.to_vec());
            auth_path_vec.push((Some(arr), dir));
        }
        // let auth_path2 = auth_path.map(|p| Some(p));
        old_coin.update_auth_path(auth_path_vec);

        Ok(old_coin)
    }

    pub(crate) async fn put_old_coin(&self, old_coin: &OldCoin) {
        // self.db.schema.put_old_coin
    }

    // pub(crate) async fn get_old_coin(&self, cm: String) -> OldCoin {
    //     // self.db.schema.get_old_coin
    //     OldCoin::default()
    // }

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
}
