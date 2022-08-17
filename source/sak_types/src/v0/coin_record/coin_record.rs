use super::CoinStatus;
use crate::AccountBalance;
use crate::TypesError;
use colored::Colorize;
use sak_crypto::Hasher;
use sak_crypto::Scalar;
use sak_crypto::ScalarExt;
use sak_proofs::{NewCoin, OldCoin};
use type_extension::U8Array;

pub type CoinIdx = u128;
pub type CmIdx = u128;
pub type CM = Scalar;

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

    pub cm_idx: CmIdx,

    pub coin_idx: Option<CoinIdx>,
}

impl CoinRecord {
    pub fn new(
        rho: u64,
        r: u64,
        s: u64,
        addr_sk: u64,
        v: u64,
        coin_idx: Option<CoinIdx>,
    ) -> Result<CoinRecord, TypesError> {
        let hasher = Hasher::new();

        let (addr_pk, addr_sk) = {
            let pk = U8Array::from_int(addr_sk);

            let addr_pk = hasher.mimc_single(&pk)?;
            let addr_sk = ScalarExt::parse_arr(&pk)?;

            (addr_pk, addr_sk)
        };

        let rho = {
            let arr = U8Array::from_int(rho as u64);

            ScalarExt::parse_arr(&arr)?
        };

        let r = {
            let arr = U8Array::from_int(r as u64);

            ScalarExt::parse_arr(&arr)?
        };

        let s = {
            let arr = U8Array::from_int(s as u64);

            ScalarExt::parse_arr(&arr)?
        };

        let v = ScalarExt::parse_u64(v)?;

        let k = hasher.comm2_scalar(r, addr_pk, rho);

        let cm = hasher.comm2_scalar(s, v, k);

        let coin = CoinRecord {
            addr_pk,
            addr_sk,
            rho,
            r,
            s,
            v,
            cm,
            coin_status: CoinStatus::Unused,
            coin_idx,
        };

        Ok(coin)
    }

    pub fn extract(&self) -> NewCoin {
        let addr_pk = self.addr_pk;
        let rho = self.rho;
        let r = self.r;
        let s = self.s;
        let v = self.v;

        NewCoin {
            addr_pk: Some(addr_pk),
            rho: Some(rho),
            r: Some(r),
            s: Some(s),
            v: Some(v),
        }
    }
}

impl std::fmt::Display for CoinRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match ScalarExt::into_u64(self.v) {
            Ok(v) => v.to_string(),
            Err(err) => format!("Invalid value, err: {:10}", err),
        };

        write!(
            f,
            "Coin [cm: {}, val: {}, status: {}]",
            &self.cm.to_string().green(),
            val,
            &self.coin_status
        )
    }
}
