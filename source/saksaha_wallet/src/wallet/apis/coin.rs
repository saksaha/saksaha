use crate::wallet::Wallet;
use crate::wallet::GAS;
use crate::WalletError;
use core::time::Duration;
use sak_contract_std::CtrRequest;
use sak_crypto::Hasher;
use sak_crypto::Scalar;
use sak_crypto::ScalarExt;
use sak_proofs::CoinProof;
use sak_proofs::OldCoin;
use sak_types::AccountBalance;
use sak_types::CoinRecord;
use sak_types::CoinStatus;
use sak_types::Sn;
use std::convert::TryInto;
use type_extension::U8Arr32;

impl Wallet {
    pub async fn get_balance(
        &self,
        acc_addr: &String,
    ) -> Result<AccountBalance, WalletError> {
        println!("wallet apis, get_balance, acc_addr: {}", acc_addr);

        let cmanager = self.get_credential_manager();
        let credential = cmanager.get_credential();

        println!("credential.acc_addr: {:?}", credential.acc_addr);
        println!("acc_addr:            {:?}", acc_addr);

        if &credential.acc_addr != acc_addr {
            return Err(format!(
                "acc addr is not correct. Candidates are: {:?}",
                cmanager.get_candidates(),
            )
            .into());
        }

        let mut balance: u64 = 0;

        for coin in self.get_db().schema.get_all_coins()? {
            let bytes = coin.v.to_bytes();

            let arr: [u8; 8] = bytes[24..].try_into()?;

            let val = u64::from_le_bytes(arr);

            balance += val;
        }

        let b = AccountBalance { val: balance };

        Ok(b)
    }

    pub async fn send_pour_tx(
        &self,
        acc_addr: String,
        ctr_addr: String,
        ctr_request: CtrRequest,
    ) -> Result<String, WalletError> {
        self.check_balance(&acc_addr).await?;

        let mut coin_manager_lock = self.get_coin_manager().write().await;

        let coin: &CoinRecord = coin_manager_lock
            .get_next_available_coin()
            .ok_or("No usable coins")?;

        let sn_1 = self.compute_sn(coin);

        let (mut new_coin_1, mut new_coin_2, cm_1, cm_2) = {
            let v = ScalarExt::into_u64(coin.v)?;

            let new_coin_1 = CoinRecord::new_random(v - GAS, None, None)?;

            let new_coin_2 = CoinRecord::new_random(0, None, None)?;

            let cm_1 = new_coin_1.cm.to_bytes();

            let cm_2 = new_coin_2.cm.to_bytes();

            (new_coin_1, new_coin_2, cm_1, cm_2)
        };

        let cm_idx = {
            let resp = saksaha::get_cm_idx(coin.cm.to_bytes()).await?;

            resp.result.ok_or("")?.cm_idx.ok_or("")?
        };

        let merkle_rt;

        let old_coin = {
            let auth_path = {
                let response = saksaha::get_auth_path(cm_idx).await?;

                let result =
                    response.result.ok_or(format!("cannot get auth path"))?;

                let auth_path = result.auth_path;

                {
                    let hasher = Hasher::new();

                    let mut curr = coin.cm.to_bytes();

                    for (_, merkle_node) in auth_path.iter().enumerate() {
                        let xl_value;
                        let xr_value;

                        let is_left: bool = merkle_node.1;

                        if is_left {
                            xl_value = merkle_node.0;
                            xr_value = curr;
                        } else {
                            xl_value = curr;
                            xr_value = merkle_node.0;
                        }

                        curr = hasher.mimc(&xl_value, &xr_value)?.to_bytes();
                    }

                    merkle_rt = curr;
                };

                auth_path
            };

            self.get_old_coin(coin, auth_path).await?
        };

        let pi = CoinProof::generate_proof_1_to_2(
            old_coin,
            new_coin_1.extract(),
            new_coin_2.extract(),
        )?;

        let mut pi_ser = Vec::new();
        pi.write(&mut pi_ser).unwrap();

        println!("[!] pi serialized, len: {}, {:?}", pi_ser.len(), pi_ser);

        let json_response = saksaha::send_tx_pour(
            sn_1,
            cm_1,
            cm_2,
            merkle_rt,
            pi_ser,
            ctr_addr,
            ctr_request,
        )
        .await?;

        let tx_hash =
            json_response.result.ok_or("Value needs to be returned")?;

        tokio::time::sleep(Duration::from_secs(10)).await;

        new_coin_1.coin_status = CoinStatus::Unconfirmed(Some(tx_hash.clone()));
        new_coin_2.coin_status = CoinStatus::Unconfirmed(Some(tx_hash.clone()));

        coin_manager_lock.insert_coin(new_coin_1.clone())?;
        coin_manager_lock.insert_coin(new_coin_2.clone())?;

        self.get_db().schema.put_coin(&new_coin_1)?;
        self.get_db().schema.put_coin(&new_coin_2)?;

        Ok("success_power".to_string())
    }

    // pub(crate) async fn update_cm(&self) {
    //     let coin_manager_lock = self.get_coin_manager().write().await;

    //     let tx_hashes = coin_manager_lock.tx_hashes.clone();
    // }

    pub(crate) async fn check_balance(
        &self,
        acc_addr: &String,
    ) -> Result<(), WalletError> {
        let my_balance = self.get_balance(acc_addr).await?;

        let is_enough_balalnce = my_balance.val > GAS;

        if !is_enough_balalnce {
            return Err(format!("you don't have enough coin").into());
        }
        Ok(())
    }

    pub(crate) async fn get_old_coin(
        &self,
        coin: &CoinRecord,
        auth_path: Vec<([u8; 32], bool)>,
    ) -> Result<OldCoin, WalletError> {
        let mut v: Vec<Option<(Scalar, bool)>> = vec![];
        for (merkle_node, dir) in auth_path {
            let s = ScalarExt::parse_arr(&merkle_node)?;
            v.push(Some((s, dir)));
        }

        let a = v.as_slice();

        let o = OldCoin {
            addr_pk: Some(coin.addr_pk),

            addr_sk: Some(coin.addr_sk),

            rho: Some(coin.rho),

            r: Some(coin.r),

            s: Some(coin.s),

            v: Some(coin.v),

            cm: Some(coin.cm),

            auth_path: a.try_into()?,
        };
        Ok(o)
    }

    pub(crate) fn compute_sn(&self, coin: &CoinRecord) -> U8Arr32 {
        let sn = {
            let addr_sk = coin.addr_sk;

            let rho = coin.rho;

            let hasher = Hasher::new();

            let s = hasher.mimc_scalar(addr_sk, rho);

            s.to_bytes()
        };

        sn
    }

    pub async fn update_coin_status(
        &self,
        acc_addr: &String,
    ) -> Result<(), WalletError> {
        let cmanager = self.get_credential_manager();
        let credential = cmanager.get_credential();

        println!("credential.acc_addr: {:?}", credential.acc_addr);
        println!("acc_addr:            {:?}", acc_addr);

        if &credential.acc_addr != acc_addr {
            return Err(format!(
                "acc addr is not correct. Candidates are: {:?}",
                cmanager.get_candidates(),
            )
            .into());
        }

        let mut old_coin_sn_vec: Vec<Sn> = Vec::new();

        let coin_manager_lock = &mut self.get_coin_manager().write().await;

        let coins_vec = coin_manager_lock.coins.clone();

        // println!("CoinManager Status: {:#?}", coin_manager_lock.coins);

        // Unconfirmed -> Unused
        for mut coin in coin_manager_lock.coins.clone().into_iter() {
            let a = coin.coin_status.clone();
            match a {
                CoinStatus::Unconfirmed(th) => match th {
                    Some(tx_hash) => {
                        let resp = saksaha::get_tx(tx_hash.clone())
                            .await?
                            .result
                            .ok_or("json_response error")?;

                        if let Some(tx) = resp.tx {
                            old_coin_sn_vec.push(tx.get_sn());

                            coin.make_status_used();

                            self.get_db().schema.raw.single_put_coin_status(
                                &coin.cm,
                                &CoinStatus::Unused,
                            )?;
                        };
                    }
                    None => {}
                },
                _ => {}
            }
        }

        // Unused -> Used
        for mut coin in coin_manager_lock.coins.clone().into_iter() {
            match coin.coin_status {
                CoinStatus::Unused => {
                    let sn = self.compute_sn(&coin);

                    if old_coin_sn_vec.contains(&sn) {
                        coin.coin_status = CoinStatus::Used;
                    }

                    self.get_db()
                        .schema
                        .raw
                        .single_put_coin_status(&coin.cm, &coin.coin_status)?;
                }

                CoinStatus::Used => {}

                CoinStatus::Unconfirmed(_) => {}
            }
        }

        println!("CoinManager Status: {:#?}", coin_manager_lock.coins);

        // =-=-= coin coin_status update (unused -> used)
        // =-=-= update new coins to be Unused, not Unconfirmed(tx_hash)

        // new_coin_1.coin_status = CoinStatus::Unused;
        // new_coin_2.coin_status = CoinStatus::Unused;

        // println!("credential.acc_addr: {:?}", credential.acc_addr);
        // println!("acc_addr:            {:?}", acc_addr);

        // if &credential.acc_addr != acc_addr {
        //     return Err(format!(
        //         "acc addr is not correct. Candidates are: {:?}",
        //         cmanager.get_candidates(),
        //     )
        //     .into());
        // }

        // let mut balance: u64 = 0;

        // for coin in self.get_db().schema.get_all_coins()? {
        //     let bytes = coin.v.to_bytes();

        //     let arr: [u8; 8] = bytes[24..].try_into()?;

        //     let val = u64::from_le_bytes(arr);

        //     balance += val;
        // }

        // let b = AccountBalance { val: balance };

        // Ok(b)

        Ok(())
    }
}
