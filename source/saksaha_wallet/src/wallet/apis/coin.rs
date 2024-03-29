use crate::wallet::Wallet;
use crate::WalletError;
use sak_contract_std::CtrRequest;
use sak_contract_std::CtrRequestData;
use sak_crypto::encode_hex;
use sak_crypto::hasher::MiMC;
use sak_crypto::MerkleTreeSim;
use sak_crypto::Scalar;
use sak_crypto::ScalarExt;
use sak_ledger_cfg::CM_TREE_DEPTH;
use sak_ledger_cfg::GAS;
use sak_ledger_testing::DUMMY_SN;
use sak_logger::debug;
use sak_proof::CoinProof;
use sak_proof_types::{NewCoin, OldCoin};
use sak_types::mock_coin_custom;
use sak_types::AccountBalance;
use sak_types::{CoinRecord, CoinStatus};
use std::convert::TryInto;

impl Wallet {
    pub async fn get_balance(&self, acc_addr: &String) -> Result<AccountBalance, WalletError> {
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

        self.update_coin_status().await?;

        let mut balance: u64 = 0;

        println!("[coin info (w/o cm)]");
        for coin in self.get_db().schema.get_all_coins()? {
            println!(
                "\
               \tcoin_status: {:?}\tvalue: {:?}\t\
                coin_idx: {:?}\tcm_idx: {:?}\t\n\
                \tcm: {}",
                coin.coin_status,
                ScalarExt::into_u64(coin.v)?,
                coin.coin_idx,
                coin.cm_idx,
                coin.cm,
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

    #[inline]
    pub(crate) async fn prepare_cm_idx(&self, coin: &CoinRecord) -> Result<u128, WalletError> {
        let cm_idx = {
            debug!("get_cm_idx call");
            let resp =
                saksaha::get_cm_idx(self.saksaha_endpoint.clone(), coin.cm.to_bytes()).await?;

            resp.result.ok_or("")?.cm_idx.ok_or("")?
        };

        Ok(cm_idx)
    }

    #[inline]
    pub(crate) async fn prepare_auth_path(
        &self,
        cm_idx: u128,
    ) -> Result<Vec<([u8; 32], bool)>, WalletError> {
        let auth_path = {
            let response = saksaha::get_auth_path(self.saksaha_endpoint.clone(), cm_idx).await?;

            let result = response.result.ok_or(format!("cannot get auth path"))?;

            result.auth_path
        };

        Ok(auth_path)
    }

    pub(crate) fn prepare_dummy_auth_path(&self) -> Vec<([u8; 32], bool)> {
        let mut auth_path = vec![];
        for i in 0..CM_TREE_DEPTH {
            auth_path.push(([0u8; 32], false));
        }
        // let auth_path = {
        //     let v = vec![
        //         ([0u8; 32], false),
        //         ([0u8; 32], false),
        //         ([0u8; 32], false),
        //         ([0u8; 32], false),
        //         ([0u8; 32], false),
        //         ([0u8; 32], false),
        //     ];

        //     v
        // };

        auth_path
    }

    pub(crate) fn prepare_merkle_rt(
        &self,
        coin: &CoinRecord,
        auth_path: Vec<([u8; 32], bool)>,
    ) -> Result<[u8; 32], WalletError> {
        let merkle_rt = {
            let hasher = MiMC::new();

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

            curr
        };

        Ok(merkle_rt)
    }

    pub(crate) fn prepare_2_new_coin_records(
        &self,
        old_value: Scalar,
    ) -> Result<(CoinRecord, CoinRecord), WalletError> {
        let new_coin_1 =
            CoinRecord::new_random(ScalarExt::into_u64(old_value)? - GAS, None, None, None)?;

        let new_coin_2 = CoinRecord::new_random(0, None, None, None)?;

        Ok((new_coin_1, new_coin_2))
    }

    // pub(crate) fn prepare_proof_1_to_2(
    //     &self,
    //     old_coin: OldCoin,
    //     new_coin_1: NewCoin,
    //     new_coin_2: NewCoin,
    // ) -> Result<Vec<u8>, WalletError> {
    //     println!("[+] making proof...");

    //     let pi = CoinProof::generate_proof_1_to_2(old_coin, new_coin_1, new_coin_2)?;

    //     let mut pi_ser = Vec::new();
    //     pi.write(&mut pi_ser).unwrap();

    //     println!("[!] pi serialized: {}", encode_hex(&pi_ser));

    //     Ok(pi_ser)
    // }

    pub(crate) fn prepare_proof_2_to_2(
        &self,
        old_coin_1: OldCoin,
        old_coin_2: OldCoin,
        new_coin_1: NewCoin,
        new_coin_2: NewCoin,
    ) -> Result<Vec<u8>, WalletError> {
        println!("[+] making proof...");

        let pi = CoinProof::generate_proof_2_to_2(old_coin_1, old_coin_2, new_coin_1, new_coin_2)?;

        let mut pi_ser = Vec::new();
        pi.write(&mut pi_ser).unwrap();

        println!("[!] pi serialized: {}", encode_hex(&pi_ser));

        Ok(pi_ser)
    }

    pub async fn send_pour_tx(
        &self,
        _acc_addr: String,
        ctr_addr: String,
        ctr_request_data: CtrRequestData,
    ) -> Result<String, WalletError> {
        let mut coin_manager_lock = self.get_coin_manager().write().await;

        let coin: &mut CoinRecord = coin_manager_lock
            .get_next_available_coin()
            .ok_or("No usable coins")?;

        let cm_idx = self.prepare_cm_idx(coin).await?;

        let auth_path = self.prepare_auth_path(cm_idx).await?;

        let merkle_rt = self.prepare_merkle_rt(coin, auth_path.clone())?;

        let old_coin = self.convert_to_old_coin(coin, auth_path)?;

        let old_sn_1 = self.compute_sn(coin);

        let dummy_coin = CoinRecord::new_dummy();

        let dummy_auth_path = self.prepare_dummy_auth_path();

        let dummy_old_coin = self.convert_to_old_coin(&dummy_coin, dummy_auth_path)?;

        let dummy_merkle_rt = sak_ledger_testing::mock_rt_1().unwrap();

        let dummy_old_sn_1 = DUMMY_SN;

        let (mut new_coin_1, mut new_coin_2) = self.prepare_2_new_coin_records(coin.v)?;

        let pi = self.prepare_proof_2_to_2(
            old_coin,
            dummy_old_coin,
            new_coin_1.extract_new_coin(),
            new_coin_2.extract_new_coin(),
        )?;

        let json_response = saksaha::send_tx_pour(
            self.saksaha_endpoint.clone(),
            vec![old_sn_1, dummy_old_sn_1],
            vec![new_coin_1.cm.to_bytes(), new_coin_2.cm.to_bytes()],
            vec![merkle_rt, dummy_merkle_rt],
            pi,
            ctr_addr,
            ctr_request_data,
        )
        .await?;

        let tx_hash = match json_response.result {
            Some(hash) => hash,
            None => {
                self.get_db().update_coin_status_to_failed(coin).await?;

                return Err(format!(
                    "Send transaction_pour response error: {:?}",
                    json_response.error
                )
                .into());
            }
        };
        // let tx_hash = json_response.result.ok_or("Send_tx_pour failed")?;

        // waiting for block is written

        {
            self.get_db()
                .update_coin_status_unused_to_unconfirmed(coin)
                .await?;
        }

        new_coin_1.update_tx_hash(tx_hash.clone());
        new_coin_2.update_tx_hash(tx_hash.clone());

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

    pub(crate) async fn check_balance(&self, acc_addr: &String) -> Result<(), WalletError> {
        let my_balance = self.get_balance(acc_addr).await?;

        let is_enough_balalnce = my_balance.val >= GAS;

        if !is_enough_balalnce {
            return Err("you don't have enough coin".into());
        }
        Ok(())
    }

    pub(crate) fn convert_to_old_coin(
        &self,
        coin: &CoinRecord,
        auth_path: Vec<([u8; CM_TREE_DEPTH as usize], bool)>,
    ) -> Result<OldCoin, WalletError> {
        let mut v: Vec<Option<(Scalar, bool)>> = vec![];
        for (merkle_node, dir) in auth_path {
            let s = ScalarExt::parse_arr(&merkle_node)?;
            v.push(Some((s, dir)));
        }

        let a = v.as_slice();

        // println!("aaa ({}): {:?}", a.len(), a);

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

    pub(crate) fn compute_sn(&self, coin: &CoinRecord) -> [u8; 32] {
        let sn = {
            let addr_sk = coin.addr_sk;

            let rho = coin.rho;

            let hasher = MiMC::new();

            let s = hasher.mimc_scalar(addr_sk, rho);

            s.to_bytes()
        };

        sn
    }

    pub async fn update_coin_status(&self) -> Result<(), WalletError> {
        let mut coin_manager_lock = self.coin_manager.write().await;

        let wallet_db = self.get_db();
        {
            let coins = &coin_manager_lock.coins;

            let old_coin_sn_vec = wallet_db
                .update_coin_status_unconfirmed_to_unused(&self.saksaha_endpoint, coins)
                .await?;

            wallet_db
                .update_coin_status_unused_to_used(old_coin_sn_vec, coins)
                .await?;
        }

        for coin in coin_manager_lock.coins.iter_mut() {
            let cm = coin.cm;

            let db_coin_status = wallet_db.schema.raw.get_coin_status(&cm)?.ok_or("FFFF")?;

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
