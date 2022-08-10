use super::WalletApis;
use crate::rpc::routes::v0::WalletSendTxRequest;
use crate::{wallet::apis::decode_hex_string_to_u64, WalletError};
use log::debug;
use sak_crypto::{
    groth16, mimc, os_rng, Bls12, Circuit, Hasher, Proof, Scalar, ScalarExt,
};
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};
use sak_types::Balance;
use saksaha::{generate_proof_1_to_2, get_auth_path};

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
        req: WalletSendTxRequest,
        // send_tx_req: SendTxRequest,
    ) -> Result<(), WalletError> {
        let WalletSendTxRequest {
            val,
            ctr_addr,
            req_type,
            args,
        } = req;

        {
            // let old_coin = prepare_old_coin(val).await?;

            // spend old coin, update state to the "used"
            // should prepare (pi, sn_1,  sn_2, cm_1, cm_2, merkle_rt) to send tx pour

            saksaha::send_tx_pour(ctr_addr, req_type, args).await?;
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
