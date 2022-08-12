use crate::WalletError;

use super::Status;
use sak_crypto::Hasher;
use sak_crypto::Scalar;
use sak_crypto::ScalarExt;
use sak_proofs::NewCoin;
use sak_types::Balance;
use sak_types::U8Array;

#[derive(Debug)]
pub(crate) struct Coin {
    pub addr_pk: Scalar,

    pub addr_sk: Scalar,

    pub rho: Scalar,

    pub r: Scalar,

    pub s: Scalar,

    pub v: Scalar,

    pub cm: Scalar,

    pub user_id: String,

    pub status: Status,
}

impl Coin {
    pub(crate) fn new(
        value: u64,
        user_id: &String,
    ) -> Result<Coin, WalletError> {
        let hasher = Hasher::new();

        let addr_sk = { U8Array::from_int(10 as u64) };

        let addr_pk = { hasher.mimc_single(&addr_sk)? };

        let rho = { U8Array::from_int(11 as u64) };

        let r = { U8Array::from_int(12 as u64) };

        let s = { U8Array::from_int(13 as u64) };

        let v = { U8Array::from_int(value) };

        let k = {
            hasher.comm2_scalar(
                ScalarExt::parse_arr(&r)?,
                addr_pk,
                ScalarExt::parse_arr(&rho)?,
            )
        };

        let cm = hasher.comm2_scalar(
            ScalarExt::parse_arr(&s)?,
            ScalarExt::parse_arr(&v)?,
            k,
        );

        let addr_sk = ScalarExt::parse_arr(&addr_sk)?;
        let rho = ScalarExt::parse_arr(&rho)?;
        let r = ScalarExt::parse_arr(&r)?;
        let s = ScalarExt::parse_arr(&s)?;
        let v = ScalarExt::parse_arr(&v)?;

        let coin = Coin {
            addr_pk,
            addr_sk,
            rho,
            r,
            s,
            v,
            cm,
            user_id: user_id.clone(),
            status: Status::Unused,
        };

        Ok(coin)
    }

    // pub(crate) fn extract(&self) -> NewCoin {
    //     let addr_pk = self.addr_pk;
    //     let rho = self.rho;
    //     let r = self.r;
    //     let s = self.s;
    //     let v = self.v;

    //     NewCoin {
    //         addr_pk,
    //         rho,
    //         r,
    //         s,
    //         v,
    //     }
    // }
}
