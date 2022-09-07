use super::CoinStatus;
use crate::CmIdx;
use crate::Sn;
use crate::TxHash;
use crate::TypesError;
use colored::Colorize;
use sak_crypto::decode_hex;
use sak_crypto::Scalar;
use sak_crypto::ScalarExt;
use sak_proofs::Hasher;
use sak_proofs::NewCoin;
use type_extension::U8Array;

pub type CoinIdx = u128;

#[derive(Debug, Clone)]
pub struct CoinRecord {
    pub addr_pk: Scalar,

    pub addr_sk: Scalar,

    pub rho: Scalar,

    pub r: Scalar,

    pub s: Scalar,

    pub v: Scalar,

    pub cm: Scalar,

    pub coin_status: CoinStatus,

    pub cm_idx: Option<CmIdx>,

    pub coin_idx: Option<CoinIdx>,

    pub tx_hash: Option<TxHash>,
}

impl CoinRecord {
    pub fn new(
        rho: u64,
        r: u64,
        s: u64,
        addr_sk: u64,
        v: u64,
        cm_idx: Option<CoinIdx>,
        coin_idx: Option<CoinIdx>,
        tx_hash: Option<TxHash>,
    ) -> Result<CoinRecord, TypesError> {
        let hasher = Hasher::new();

        let (addr_pk, addr_sk) = {
            let a_sk = U8Array::from_int(addr_sk);

            let addr_sk = ScalarExt::parse_arr(&a_sk)?;

            let addr_pk = hasher.mimc_single(&a_sk)?;

            (addr_pk, addr_sk)
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
            // change it!
            coin_status: CoinStatus::Unused,
            // coin_status: CoinStatus::Unused,
            cm_idx,
            coin_idx,
            tx_hash,
        };

        Ok(coin)
    }

    pub fn new_random(
        // rho: u64,
        // r: u64,
        // s: u64,
        // addr_sk: u64,
        v: u64,
        cm_idx: Option<CoinIdx>,
        coin_idx: Option<CoinIdx>,
        tx_hash: Option<TxHash>,
    ) -> Result<CoinRecord, TypesError> {
        let hasher = Hasher::new();

        // U8Array::from_int()

        let (addr_pk, addr_sk) = {
            let pk = U8Array::from_int(sak_crypto::rand() as u64);

            let addr_pk = hasher.mimc_single(&pk)?;
            let addr_sk = ScalarExt::parse_arr(&pk)?;

            (addr_pk, addr_sk)
        };

        let rho = {
            let arr = U8Array::from_int(sak_crypto::rand() as u64);

            ScalarExt::parse_arr(&arr)?
        };

        let r = {
            let arr = U8Array::from_int(sak_crypto::rand() as u64);

            ScalarExt::parse_arr(&arr)?
        };

        let s = {
            let arr = U8Array::from_int(sak_crypto::rand() as u64);

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
            coin_status: CoinStatus::Unconfirmed,
            cm_idx,
            coin_idx,
            tx_hash,
        };

        Ok(coin)
    }

    pub fn new_dummy() -> CoinRecord {
        let hasher = Hasher::new();

        let addr_sk = Scalar::default();
        let addr_pk = hasher.mimc_single_scalar(addr_sk).unwrap();
        let rho = Scalar::default();
        let r = Scalar::default();
        let s = Scalar::default();
        let v = Scalar::default();

        let k = hasher.comm2_scalar(r, addr_pk, rho);
        let cm = hasher.comm2_scalar(s, v, k);

        let coin_status = CoinStatus::Unused;
        let cm_idx = None;
        // let cm_idx = Some(0);
        let coin_idx = None;

        let tx_hash = None;

        CoinRecord {
            addr_pk,
            addr_sk,
            rho,
            r,
            s,
            v,
            cm,
            coin_status,
            cm_idx,
            coin_idx,
            tx_hash,
        }
    }

    pub fn extract_new_coin(&self) -> NewCoin {
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

    pub fn compute_sn(&self) -> Sn {
        let sn = {
            let addr_sk = self.addr_sk;

            let rho = self.rho;

            let hasher = Hasher::new();

            let s = hasher.mimc_scalar(addr_sk, rho);

            s.to_bytes()
        };

        sn
    }

    pub fn set_coin_status_to(&mut self, status: CoinStatus) {
        self.coin_status = status;
    }

    pub fn update_tx_hash(&mut self, tx_hash: String) {
        self.tx_hash = Some(tx_hash);
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
