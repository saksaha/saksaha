use super::CoinStatus;
use crate::Balance;
use crate::TypesError;
use sak_crypto::Hasher;
use sak_crypto::Scalar;
use sak_crypto::ScalarExt;
use sak_proofs::NewCoin;
use type_extension::U8Array;

#[derive(Debug)]
pub struct CoinRecord {
    pub addr_pk: Scalar,

    pub addr_sk: Scalar,

    pub rho: Scalar,

    pub r: Scalar,

    pub s: Scalar,

    pub v: Scalar,

    pub cm: Scalar,

    pub coin_status: CoinStatus,
}

impl CoinRecord {
    pub fn new(value: u64) -> Result<CoinRecord, TypesError> {
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

        let coin = CoinRecord {
            addr_pk,
            addr_sk,
            rho,
            r,
            s,
            v,
            cm,
            coin_status: CoinStatus::Unused,
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
