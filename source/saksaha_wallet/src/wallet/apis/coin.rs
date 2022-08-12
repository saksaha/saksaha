use super::WalletApis;
use crate::rpc::routes::v0::WalletSendTxRequest;
use crate::types::Coin;
use crate::WalletError;
use crate::{types::Status, wallet::apis::decode_hex_string_to_u64};
use sak_crypto::Hasher;
use sak_crypto::Scalar;
use sak_proofs::OldCoin;
use sak_types::Balance;
use saksaha::{generate_proof_1_to_2, get_auth_path};
pub const GAS: u64 = 10;

impl WalletApis {
    pub async fn get_balance(
        &self,
        id: &String,
        _key: &String,
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

            let _status = match self.db.schema.get_status(&cm).await? {
                Some(s) => {
                    if s == Status::Used {
                        continue;
                    }
                }
                None => return Err(format!("Failed to get status").into()),
            };

            if user == *id {
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
            val,
            ctr_addr,
            req_type,
            args,
        } = req;

        let id = String::from("user_1");
        let key = String::from("user1pw");

        self.check_enough_balance(&id, &key).await?;

        let cm_idx = match self.db.schema.get_latest_cm_idx()? {
            Some(i) => i,
            None => {
                return Err(format!("Wallet is empty, cannot get any cm").into())
            }
        };

        let cm = match self.db.schema.get_cm(&cm_idx)? {
            Some(c) => c,
            None => return Err(format!("cannot get cm").into()),
        };

        let (old_coin, old_coin_v) = {
            let auth_path = {
                let response = saksaha::get_auth_path(cm_idx).await?;

                let result =
                    response.result.ok_or(format!("cannot get auth path"))?;

                result.auth_path
            };

            let old_coin = self.get_old_coin(cm_idx, auth_path).await?;

            let old_coin_v = match old_coin.v {
                Some(v) => decode_hex_string_to_u64(&v.to_string()).await?,
                None => return Err(format!("coin has no value").into()),
            };

            (old_coin, old_coin_v)
        };

        {
            let addr_sk = match old_coin.addr_sk {
                Some(s) => s,
                None => return Err(format!("cannot get addr_sk").into()),
            };

            let rho = match old_coin.rho {
                Some(r) => r,
                None => return Err(format!("cannot get rho").into()),
            };

            let hasher = Hasher::new();

            let sn_1_old = hasher.mimc_scalar(addr_sk, rho);

            let new_coin_1 = Coin::new(old_coin_v - GAS, &id);

            let new_coin_2 = Coin::new(0, &id);

            let pi = generate_proof_1_to_2(
                old_coin,
                new_coin_1.extract(),
                new_coin_2.extract(),
            )
            .await?;

            saksaha::send_tx_pour(
                // pi,
                // sn_1_old.to_bytes().into(),
                // new_coin_1.cm,
                // new_coin_2.cm,
                // merkle_rt,
                ctr_addr, req_type, args,
            )
            .await?;

            self.set_status_used(&cm, &Status::Used).await?;
        }

        Ok(())
    }

    pub(crate) async fn check_enough_balance(
        &self,
        id: &String,
        key: &String,
    ) -> Result<(), WalletError> {
        let my_balance = self.get_balance(id, key).await?;
        let check_enough_balalnce = my_balance.val > GAS;
        if !check_enough_balalnce {
            return Err(format!("don't have enough coin").into());
        }
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

        // unwrap should be resolved
        let mut auth_path_vec = vec![];
        for (arr, dir) in auth_path {
            let node = Scalar::from_bytes(&arr).unwrap();
            auth_path_vec.push(Some((node, dir)));
        }
        old_coin.update_auth_path(auth_path_vec.try_into().unwrap());

        Ok(old_coin)
    }

    pub(crate) async fn set_status_used(
        &self,
        cm: &String,
        status: &Status,
    ) -> Result<(), WalletError> {
        self.db.schema.put_status(cm, status).await?;
        Ok(())
    }
}
