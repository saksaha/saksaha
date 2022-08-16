use crate::wallet::Wallet;
use crate::WalletError;
use sak_contract_std::CtrRequest;
use sak_crypto::Hasher;
use sak_crypto::Scalar;
use sak_proofs::OldCoin;
use sak_types::AccountBalance;
use sak_types::CoinRecord;
use sak_types::CoinStatus;
use std::convert::TryInto;
use type_extension::U8Array;

pub const GAS: u64 = 10;

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
        let coin_manager = self.get_coin_manager();

        // let coin: CoinRecord = coin_manager.get_next_available_coin()?;

        // let sn_1 = {
        //     let some_hashed_result = 0;
        // };

        // let new_coin_1 = CoinRecord::new()?;
        // let new_coin_2 = CoinRecord::new()?;

        // let cm_1 = new_coin_1.cm;
        // let cm_2 = new_coin_2.cm;

        // let pi =
        //     saksaha::generate_proof_1_to_2(coin_1_old, coin_1_new, coin_2_new)
        //         .await?;

        // // send
        // let json_response = saksaha::send_tx_pour(
        //     U8Array::new_empty_32(),
        //     U8Array::new_empty_32(),
        //     U8Array::new_empty_32(),
        //     U8Array::new_empty_32(),
        //     vec![],
        //     ctr_addr,
        //     ctr_request,
        // )
        // .await?;

        // let res = json_response.result.ok_or("Value needs to be returned")?;

        // coin_manager.add_coin()?;
        ////////////////////////////////////////////////

        // self.check_enough_balance(&id, &key).await?;

        // let cm_idx = match self.db.schema.get_latest_cm_idx()? {
        //     Some(i) => i,
        //     None => {
        //         return Err(format!("Wallet is empty, cannot get any cm").into())
        //     }
        // };

        // let cm = match self.db.schema.get_cm(&cm_idx)? {
        //     Some(c) => c,
        //     None => return Err(format!("cannot get cm").into()),
        // };

        // let (old_coin, old_coin_v) = {
        //     let auth_path = {
        //         let response = saksaha::get_auth_path(cm_idx).await?;

        //         let result =
        //             response.result.ok_or(format!("cannot get auth path"))?;

        //         result.auth_path
        //     };

        //     let old_coin = self.get_old_coin(cm_idx, auth_path).await?;

        //     let old_coin_v = match old_coin.v {
        //         Some(v) => decode_hex_string_to_u64(&v.to_string()).await?,
        //         None => return Err(format!("coin has no value").into()),
        //     };

        //     (old_coin, old_coin_v)
        // };

        // {
        //     let addr_sk = match old_coin.addr_sk {
        //         Some(s) => s,
        //         None => return Err(format!("cannot get addr_sk").into()),
        //     };

        //     let rho = match old_coin.rho {
        //         Some(r) => r,
        //         None => return Err(format!("cannot get rho").into()),
        //     };

        //     let hasher = Hasher::new();

        //     let sn_1_old = hasher.mimc_scalar(addr_sk, rho);

        //     let new_coin_1 = CoinRecord::new(old_coin_v - GAS, &id);

        //     let new_coin_2 = CoinRecord::new(0, &id);

        //     let pi = generate_proof_1_to_2(
        //         old_coin,
        //         new_coin_1.extract(),
        //         new_coin_2.extract(),
        //     )
        //     .await?;

        Ok("power".to_string())
    }
}

// pub(crate) async fn check_enough_balance(
//     &self,
//     id: &String,
//     key: &String,
// ) -> Result<(), WalletError> {
//     let my_balance = self.get_balance(id, key).await?;
//     let check_enough_balalnce = my_balance.val > GAS;
//     if !check_enough_balalnce {
//         return Err(format!("don't have enough coin").into());
//     }
//     Ok(())
// }

// pub(crate) async fn get_old_coin(
//     &self,
//     // cm_idx: u128,
//     auth_path: Vec<([u8; 32], bool)>,
// ) -> Result<OldCoin, WalletError> {
//     let cm: String = match self.db.schema.get_cm(&cm_idx) {
//         Ok(c) => match c {
//             Some(c) => c,
//             None => {
//                 return Err(format!(
//                     "No cm has been found at idx: {:?}",
//                     cm_idx
//                 )
//                 .into())
//             }
//         },
//         Err(err) => {
//             return Err(format!("Failed to get cm, err: {:?}", err).into())
//         }
//     };

//     let mut old_coin = self.db.schema.get_coin(&cm)?;

//     // unwrap should be resolved
//     let mut auth_path_vec = vec![];
//     for (arr, dir) in auth_path {
//         let node = Scalar::from_bytes(&arr).unwrap();
//         auth_path_vec.push(Some((node, dir)));
//     }
//     old_coin.update_auth_path(auth_path_vec.try_into().unwrap());

//     Ok(old_coin)
// }

// pub(crate) async fn set_status_used(
//     &self,
//     cm: &String,
//     status: &CoinStatus,
// ) -> Result<(), WalletError> {
//     self.db.schema.put_coin_status(cm, status).await?;
