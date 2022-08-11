use super::apis::{self, WalletApis};
use crate::{credential::Credential, db::WalletDB, types::Coin, WalletError};
use futures::sink::Send;
use log::debug;
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};

use sak_crypto::{Hasher, ScalarExt};
use sak_types::U8Array;

pub(crate) struct Wallet {
    pub apis: WalletApis,
    credential: Credential,
}

impl Wallet {
    pub async fn init(
        app_prefix: String,
        credential: Credential,
    ) -> Result<Wallet, WalletError> {
        let wallet_db = WalletDB::init(&app_prefix).await?;

        let apis = WalletApis { db: wallet_db };

        let w = Wallet { credential, apis };

        {
            init_for_demo(&w).await?;
        }

        Ok(w)
    }
}

// pub struct SendTxPourRequest {
//     pi: U8Array,
//     sn_1: U8Array,
//     sn_2: U8Array,
//     cm_1: U8Array,
//     cm_2: U8Array,
//     merkle_rt: U8Array,
// }

async fn init_for_demo(wallet: &Wallet) -> Result<(), WalletError> {
    {
        let user_id = "user_1".to_string();
        let value = 100;

        let coin = gen_random_coin_with_value(user_id, value);

        debug!("[demo coin: user_1] {:#?}", coin);

        wallet
            .apis
            .db
            .schema
            .put_coin_data(
                &coin.cm.unwrap().to_string(),
                &coin.rho.unwrap().to_string(),
                &coin.r.unwrap().to_string(),
                &coin.s.unwrap().to_string(),
                &coin.v.unwrap().to_string(),
                &coin.addr_pk.unwrap().to_string(),
                &coin.addr_sk.unwrap().to_string(),
                &coin.user_id.unwrap().to_string(),
                &coin.status.unwrap().to_string(),
                &0,
            )
            .await?;
    }

    {
        let user_id = "user_2".to_string();
        let value = 100;

        let coin = gen_random_coin_with_value(user_id, value);

        debug!("[demo coin: user_2] {:#?}", coin);

        wallet
            .apis
            .db
            .schema
            .put_coin_data(
                &coin.cm.unwrap().to_string(),
                &coin.rho.unwrap().to_string(),
                &coin.r.unwrap().to_string(),
                &coin.s.unwrap().to_string(),
                &coin.v.unwrap().to_string(),
                &coin.addr_pk.unwrap().to_string(),
                &coin.addr_sk.unwrap().to_string(),
                &coin.user_id.unwrap().to_string(),
                &coin.status.unwrap().to_string(),
                &1,
            )
            .await?;
    }

    Ok(())
}

fn gen_random_coin_with_value(user_id: String, value: u64) -> Coin {
    let hasher = Hasher::new();

    let addr_sk = U8Array::from_int(sak_crypto::rand() as u64).to_owned();
    let addr_pk = hasher.mimc_single(&addr_sk).unwrap();
    let rho = U8Array::from_int(sak_crypto::rand() as u64);
    let r = U8Array::from_int(sak_crypto::rand() as u64);
    let s = U8Array::from_int(sak_crypto::rand() as u64);
    let v = U8Array::from_int(value as u64);
    let k = hasher.comm2_scalar(
        ScalarExt::parse_arr(&r).unwrap(),
        addr_pk,
        ScalarExt::parse_arr(&rho).unwrap(),
    );
    let cm = hasher.comm2_scalar(
        ScalarExt::parse_arr(&s).unwrap(),
        ScalarExt::parse_arr(&v).unwrap(),
        k,
    );
    let status = true;

    Coin {
        addr_pk: Some(addr_pk),
        addr_sk: Some(ScalarExt::parse_arr(&addr_sk).unwrap()),
        rho: Some(ScalarExt::parse_arr(&rho).unwrap()),
        r: Some(ScalarExt::parse_arr(&r).unwrap()),
        s: Some(ScalarExt::parse_arr(&s).unwrap()),
        v: Some(ScalarExt::parse_arr(&v).unwrap()),
        cm: Some(cm),
        user_id: Some(user_id),
        status: Some(status),
    }
}
