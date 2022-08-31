use crate::wallet::Wallet;
use crate::wallet::GAS;
use crate::WalletError;
use core::time::Duration;
use sak_contract_std::CtrRequest;
use sak_crypto::Scalar;
use sak_crypto::ScalarExt;
use sak_proofs::CoinProof;
use sak_proofs::Hasher;
use sak_proofs::OldCoin;
use sak_types::AccountBalance;
use sak_types::CoinRecord;
use sak_types::CoinStatus;
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

        // println!("[+] check account address..");
        // println!("\tcredential.acc_addr: {:?}", credential.acc_addr);
        // println!("\tacc_addr:            {:?}", acc_addr);

        if &credential.acc_addr != acc_addr {
            return Err(format!(
                "acc addr is not correct. Candidates are: {:?}",
                cmanager.get_candidates(),
            )
            .into());
        }

        self.update_coin_status(acc_addr).await?;

        let mut balance: u64 = 0;

        println!("[coin info (w/o cm)]");
        for coin in self.get_db().schema.get_all_coins()? {
            println!(
                // cm: {:?}\n \
                "\
                \tcoin_status: {:?}\tvalue: {:?}\t\
                coin_idx: {:?}\tcm_idx: {:?}\t",
                // coin.cm,
                coin.coin_status,
                ScalarExt::into_u64(coin.v)?,
                coin.coin_idx,
                coin.cm_idx,
            );

            if coin.coin_status == CoinStatus::Unused {
                let bytes = coin.v.to_bytes();

                let arr: [u8; 8] = bytes[24..].try_into()?;

                let val = u64::from_le_bytes(arr);

                balance += val;
            }
        }

        let b = AccountBalance { val: balance };

        Ok(b)
    }

    // a) Take ctr state manipulation meta from user

    // b) Grab from the wallet an "old" available (unused) coin with the least
    //    value (to write tx)

    // c) Create N number of new coins (with random values associated)

    // d) Generate proof using new coins and an old coin

    // e) Request to send tx to the network and get tx_hash associated with it

    // f) Store new coins into the wallet (with tx hash)
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

        let (mut new_coin_1, mut new_coin_2) = {
            let v = ScalarExt::into_u64(coin.v)?;

            let new_coin_1 =
                CoinRecord::new_random(v - GAS, Some(0), None, None)?;

            let new_coin_2 = CoinRecord::new_random(0, Some(1), None, None)?;

            (new_coin_1, new_coin_2)
        };

        let cm_idx = {
            let resp = saksaha::get_cm_idx(
                self.saksaha_endpoint.clone(),
                coin.cm.to_bytes(),
            )
            .await?;

            resp.result.ok_or("")?.cm_idx.ok_or("")?
        };

        let merkle_rt;

        let old_coin = {
            let auth_path = {
                let response = saksaha::get_auth_path(
                    self.saksaha_endpoint.clone(),
                    cm_idx,
                )
                .await?;

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

        println!("[+] making proof...");

        let pi = CoinProof::generate_proof_1_to_2(
            old_coin,
            new_coin_1.extract(),
            new_coin_2.extract(),
        )?;

        let mut pi_ser = Vec::new();
        pi.write(&mut pi_ser).unwrap();

        println!("[!] pi serialized, len: {}", pi_ser.len());

        let json_response = saksaha::send_tx_pour(
            self.saksaha_endpoint.clone(),
            sn_1,
            vec![new_coin_1.cm.to_bytes(), new_coin_2.cm.to_bytes()],
            merkle_rt,
            pi_ser,
            ctr_addr,
            ctr_request,
        )
        .await?;

        let tx_hash =
            json_response.result.ok_or("Value needs to be returned")?;

        // waiting for block is written
        tokio::time::sleep(Duration::from_millis(6000)).await;

        new_coin_1.tx_hash = Some(tx_hash.clone());
        new_coin_2.tx_hash = Some(tx_hash);

        {
            self.get_db().schema.put_coin(&new_coin_1)?;

            self.get_db().schema.put_coin(&new_coin_2)?;

            println!("[+] new coins have been stored in db");
        }

        {
            coin_manager_lock.put_coin(new_coin_1)?;

            coin_manager_lock.put_coin(new_coin_2)?;

            println!("[+] new coins have been stored in coin_manager");
        }

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

    // pub async fn update_coin_status(
    //     &self,
    //     acc_addr: &String,
    // ) -> Result<(), WalletError> {
    //     println!("[update_coin_status] starts");

    //     // {
    //     //     // check credential
    //     //     let cmanager = self.get_credential_manager();
    //     //     let credential = cmanager.get_credential();

    //     //     println!("credential.acc_addr: {:?}", credential.acc_addr);
    //     //     println!("acc_addr:            {:?}", acc_addr);

    //     //     if &credential.acc_addr != acc_addr {
    //     //         return Err(format!(
    //     //             "acc addr is not correct. Candidates are: {:?}",
    //     //             cmanager.get_candidates(),
    //     //         )
    //     //         .into());
    //     //     }
    //     // }

    //     let coin_manager_lock = self.get_coin_manager().write().await;

    //     let coins = coin_manager_lock.coins.clone();

    //     let wallet_db = self.get_db();

    //     {
    //         // update DB first
    //         let old_coin_sn_vec = wallet_db
    //             .update_coin_status_unconfirmed_to_unused(&coins)
    //             .await?;

    //         wallet_db
    //             .update_coin_status_unused_to_used(old_coin_sn_vec, &coins)
    //             .await?;
    //     }

    //     println!("\t[+] Coin Status has been updated in DB.");

    //     tokio::time::sleep(Duration::from_secs(10)).await;

    //     // coin_manager should update `coin_status` from `DB`
    //     {
    //         // let db_coins = self.get_db().schema.get_all_coins()?;

    //         for mut coin in coins.iter() {
    //             let cm = coin.cm;

    //             let db_coin_status = self
    //                 .get_db()
    //                 .schema
    //                 .raw
    //                 .get_coin_status(&cm)?
    //                 .ok_or("FFFF")?;

    //             if coin.coin_status != db_coin_status {
    //                 coin.coin_status = db_coin_status;
    //             }
    //         }
    //     }

    //     println!("\t[+] Coin Status has been updated in Coin Manager.");

    //     Ok(())
    // }

    pub async fn update_coin_status(
        &self,
        _acc_addr: &String,
    ) -> Result<(), WalletError> {
        let mut coin_manager_lock = self.coin_manager.write().await;

        let wallet_db = self.get_db();
        {
            let coins = coin_manager_lock.coins.clone();

            let old_coin_sn_vec = wallet_db
                .update_coin_status_unconfirmed_to_unused(
                    self.saksaha_endpoint.clone(),
                    &coins,
                )
                .await?;

            wallet_db
                .update_coin_status_unused_to_used(old_coin_sn_vec, &coins)
                .await?;
        }

        for coin in coin_manager_lock.coins.iter_mut() {
            let cm = coin.cm;

            let db_coin_status =
                wallet_db.schema.raw.get_coin_status(&cm)?.ok_or("FFFF")?;

            if coin.coin_status != db_coin_status {
                coin.coin_status = db_coin_status;
            }

            let db_coin_cm_idx = wallet_db.schema.raw.get_cm_idx(&cm)?;

            if coin.cm_idx != db_coin_cm_idx {
                coin.cm_idx = db_coin_cm_idx;
            }
        }

        Ok(())
    }
}
