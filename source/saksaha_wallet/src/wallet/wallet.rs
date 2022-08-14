use super::apis::{self, WalletApis};
use crate::{credential::WalletCredential, db::WalletDB, WalletError};
use futures::sink::Send;
use log::debug;
use sak_crypto::{Hasher, ScalarExt};
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};
use sak_types::{CoinRecord, CoinStatus};
use type_extension::U8Array;

pub(crate) struct Wallet {
    pub apis: WalletApis,
    credential: WalletCredential,
}

impl Wallet {
    pub async fn init(
        app_prefix: String,
        credential: WalletCredential,
    ) -> Result<Wallet, WalletError> {
        let wallet_db = WalletDB::init(&app_prefix)?;

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
) -> Result<CoinRecord, WalletError> {
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

    let coin_status = CoinStatus::Unused;

    let coin = CoinRecord {
        addr_pk,
        addr_sk,
        rho,
        r,
        s,
        v,
        cm,
        user_id,
        coin_status,
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

        // wallet
        //     .apis
        //     .db
        //     .schema
        //     .put_coin(
        //         &coin.cm,
        //         &coin.rho,
        //         &coin.r,
        //         &coin.s,
        //         &coin.v,
        //         &coin.addr_pk,
        //         &coin.addr_sk,
        //         &coin.user_id,
        //         &coin.status,
        //     )
        //     .await?;
    }

    {
        let user_id = "user_2".to_string();
        let value = 100;

        let coin =
            gen_coin_with_params(0x21, 0x22, 0x23, 0x24, value, user_id)?;

        debug!("[demo coin: user_2] {:#?}", coin);

        // wallet
        //     .apis
        //     .db
        //     .schema
        //     .put_coin(
        //         &coin.cm,
        //         &coin.rho,
        //         &coin.r,
        //         &coin.s,
        //         &coin.v,
        //         &coin.addr_pk,
        //         &coin.addr_sk,
        //         &coin.user_id,
        //         &coin.status,
        //     )
        //     .await?;
    }

    Ok(())
}
