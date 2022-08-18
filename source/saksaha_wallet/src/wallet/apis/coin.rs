use crate::wallet::Wallet;
use crate::wallet::GAS;
use crate::WalletError;
use sak_contract_std::CtrRequest;
use sak_crypto::Hasher;
use sak_crypto::Scalar;
use sak_crypto::ScalarExt;
use sak_proofs::OldCoin;
use sak_types::AccountBalance;
use sak_types::CoinRecord;
use std::convert::TryInto;
use type_extension::U8Array;

impl Wallet {
    pub async fn get_balance(
        &self,
        acc_addr: &String,
    ) -> Result<AccountBalance, WalletError> {
        println!("wallet apis, get_balance, acc_addr: {}", acc_addr);

        let cmanager = self.get_credential_manager();
        let credential = cmanager.get_credential();

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

    pub async fn send_tx(
        &self,
        _acc_addr: String,
        ctr_addr: String,
        ctr_request: CtrRequest,
    ) -> Result<String, WalletError> {
        // self.check_enough_balance(&acc_addr).await?;

        let coin_manager_lock = self.get_coin_manager().write().await;

        let coin: &CoinRecord = coin_manager_lock
            .get_next_available_coin()
            .ok_or("No usable coins")?;

        let sn_1 = {
            let addr_sk = coin.addr_sk;

            let rho = coin.rho;

            let hasher = Hasher::new();

            let sn_1 = hasher.mimc_scalar(addr_sk, rho);

            sn_1.to_bytes()
        };

        let (new_coin_1, new_coin_2, cm_1, cm_2) = {
            let v = ScalarExt::into_u64(coin.v)?;

            let new_coin_1 = CoinRecord::new(
                0x101,
                0x102,
                0x103,
                0x104,
                v - GAS,
                None,
                None,
            )?;

            let new_coin_2 =
                CoinRecord::new(0x201, 0x202, 0x203, 0x204, 0, None, None)?;

            let cm_1 = new_coin_1.cm.to_bytes();

            let cm_2 = new_coin_2.cm.to_bytes();

            (new_coin_1, new_coin_2, cm_1, cm_2)
        };

        let cm_idx = {
            println!("coin.cm_idx: {:?}", coin.cm_idx);

            let c = coin.cm_idx.ok_or("cannot get cm_idx")?;

            c
        };

        let old_coin = {
            let auth_path = {
                let response = saksaha::get_auth_path(cm_idx).await?;

                let result =
                    response.result.ok_or(format!("cannot get auth path"))?;

                result.auth_path
            };
            self.get_old_coin(coin, auth_path).await?
        };

        let pi = saksaha::generate_proof_1_to_2(
            old_coin,
            new_coin_1.extract(),
            new_coin_2.extract(),
        )
        .await?;

        let mut pi_ser = Vec::new();
        pi.write(&mut pi_ser).unwrap();

        println!("[!] pi serialized, len: {}, {:?}", pi_ser.len(), pi_ser);

        // // send
        let json_response = saksaha::send_tx_pour(
            sn_1,
            cm_1,
            cm_2,
            U8Array::new_empty_32(), // merkle_rt
            pi_ser,
            ctr_addr,
            ctr_request,
        )
        .await?;

        let res = json_response.result.ok_or("Value needs to be returned")?;

        println!("res : {:?}", res);

        Ok("success_power".to_string())
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

    pub(crate) async fn _check_enough_balance(
        &self,
        acc_addr: &String,
    ) -> Result<(), WalletError> {
        let my_balance = self.get_balance(acc_addr).await?;
        let check_enough_balalnce = my_balance.val > GAS;
        if !check_enough_balalnce {
            return Err(format!("don't have enough coin").into());
        }
        Ok(())
    }

    // pub(crate) async fn set_status_used(
    //     &self,
    //     cm: &String,
    //     status: &CoinStatus,
    // ) -> Result<(), WalletError> {
    //     self.db.schema.put_coin_status(cm, status).await?;
}
