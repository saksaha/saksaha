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
) -> Result<Coin, WalletError> {
    let hasher = Hasher::new();

    let (addr_sk, addr_pk) = {
        let addr_sk = U8Array::from_int(addr_sk);
        let addr_pk = hasher.mimc_single(&addr_sk)?;

        let addr_sk_scalar = ScalarExt::parse_arr(&addr_sk)?;

        (addr_sk_scalar, addr_pk)
    };

    let rho = {
        let arr = U8Array::from_int(rho);
        ScalarExt::parse_arr(&arr)?
    };

    let r = {
        let arr = U8Array::from_int(r);
        ScalarExt::parse_arr(&arr)?
    };

    let s = {
        let arr = U8Array::from_int(s);
        ScalarExt::parse_arr(&arr)?
    };

    let v = {
        let arr = U8Array::from_int(v);
        ScalarExt::parse_arr(&arr)?
    };

    let k = hasher.comm2_scalar(r, addr_pk, rho);

    let cm = hasher.comm2_scalar(s, v, k);

    let status = Status::Unused;

    let coin = Coin {
        addr_pk,
        addr_sk,
        rho,
        r,
        s,
        v,
        cm,
        user_id,
        status,
    };

    Ok(coin)
}

async fn init_for_demo(wallet: &Wallet) -> Result<(), WalletError> {
    {
        let user_id = "user_1".to_string();
        let value = 100;

        let coin =
            gen_coin_with_params(0x11, 0x12, 0x13, 0x14, value, user_id)?;

        debug!("[demo coin: user_1] {:#?}", coin);

        // let cm = &coin.cm;
        // let rho = &coin.rho.ok_or("rho should exist")?;
        // let r = &coin.r.ok_or("r should exist")?;
        // let s = &coin.s.ok_or("s should exist")?;
        // let v = &coin.v.ok_or("v should exist")?;
        // let addr_pk = &coin.addr_pk.ok_or("addr_pk should exist")?;
        // let addr_sk = &coin.addr_sk.ok_or("addr_sk should exist")?;
        // let user_id = &coin.user_id.ok_or("user_id should exist")?;
        // let status = &coin.status.ok_or("status should exist")?;

        wallet
            .apis
            .db
            .schema
            .put_coin(
                &coin.cm,
                &coin.rho,
                &coin.r,
                &coin.s,
                &coin.v,
                &coin.addr_pk,
                &coin.addr_sk,
                &coin.user_id,
                &coin.status,
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
