use super::apis::{self, WalletApis};
use crate::{
    credential::Credential,
    db::WalletDB,
    types::{Coin, Status},
    WalletError,
};
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
fn gen_coin_with_params(
    rho: u64,
    r: u64,
    s: u64,
    addr_sk: u64,
    v: u64,
    user_id: String,
) -> Coin {
    let hasher = Hasher::new();

    let addr_sk = U8Array::from_int(addr_sk).to_owned();
    let addr_pk = hasher.mimc_single(&addr_sk).unwrap();
    let rho = U8Array::from_int(rho);
    let r = U8Array::from_int(r);
    let s = U8Array::from_int(s);
    let v = U8Array::from_int(v);
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
    let status = Status::Unused;

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

async fn init_for_demo(wallet: &Wallet) -> Result<(), WalletError> {
    {
        let user_id = "user_1".to_string();
        let value = 100;

        let coin = gen_coin_with_params(0x11, 0x12, 0x13, 0x14, value, user_id);

        debug!("[demo coin: user_1] {:#?}", coin);

        let cm = &coin.cm.ok_or("cm should exist")?;
        let rho = &coin.rho.ok_or("rho should exist")?;
        let r = &coin.r.ok_or("r should exist")?;
        let s = &coin.s.ok_or("s should exist")?;
        let v = &coin.v.ok_or("v should exist")?;
        let addr_pk = &coin.addr_pk.ok_or("addr_pk should exist")?;
        let addr_sk = &coin.addr_sk.ok_or("addr_sk should exist")?;
        let user_id = &coin.user_id.ok_or("user_id should exist")?;
        let status = &coin.status.ok_or("status should exist")?;

        wallet
            .apis
            .db
            .schema
            .put_coin(
                cm, rho, r, s, v, addr_pk, addr_sk, user_id, status,
                // &coin.cm.unwrap().to_string(),
                // &coin.rho.unwrap().to_string(),
                // &coin.r.unwrap().to_string(),
                // &coin.s.unwrap().to_string(),
                // &coin.v.unwrap().to_string(),
                // &coin.addr_pk.unwrap().to_string(),
                // &coin.addr_sk.unwrap().to_string(),
                // &coin.user_id.unwrap().to_string(),
                // &coin.status.unwrap(),
                &0,
            )
            .await?;
    }
    {
        let user_id = "user_2".to_string();
        let value = 100;

        let coin = gen_coin_with_params(0x21, 0x22, 0x23, 0x24, value, user_id);

        debug!("[demo coin: user_2] {:#?}", coin);

        wallet
            .apis
            .db
            .schema
            .put_coin(
                &coin.cm.unwrap().to_string(),
                &coin.rho.unwrap().to_string(),
                &coin.r.unwrap().to_string(),
                &coin.s.unwrap().to_string(),
                &coin.v.unwrap().to_string(),
                &coin.addr_pk.unwrap().to_string(),
                &coin.addr_sk.unwrap().to_string(),
                &coin.user_id.unwrap().to_string(),
                &coin.status.unwrap(),
                &1,
            )
            .await?;
    }

    Ok(())
}
