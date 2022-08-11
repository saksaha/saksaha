use super::Status;
use sak_crypto::Hasher;
use sak_crypto::Scalar;
use sak_crypto::ScalarExt;
use sak_types::Balance;
use sak_types::U8Array;

#[derive(Debug)]
pub(crate) struct Coin {
    pub addr_pk: Option<Scalar>,

    pub addr_sk: Option<Scalar>,

    pub rho: Option<Scalar>,

    pub r: Option<Scalar>,

    pub s: Option<Scalar>,

    pub v: Option<Scalar>,

    pub cm: Option<Scalar>,

    pub user_id: Option<String>,

    pub status: Option<Status>,
}

impl Coin {
    pub(crate) fn new(value: u64, user_id: String) -> Coin {
        let hasher = Hasher::new();

        let addr_sk = U8Array::from_int(sak_crypto::rand() as u64).to_owned();
        let addr_pk = hasher.mimc_single(&addr_sk).unwrap();
        let rho = U8Array::from_int(sak_crypto::rand() as u64);
        let r = U8Array::from_int(sak_crypto::rand() as u64);
        let s = U8Array::from_int(sak_crypto::rand() as u64);
        let v = U8Array::from_int(value);

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

        let addr_sk = ScalarExt::parse_arr(&addr_sk).unwrap();
        let rho = ScalarExt::parse_arr(&rho).unwrap();
        let r = ScalarExt::parse_arr(&r).unwrap();
        let s = ScalarExt::parse_arr(&s).unwrap();
        let v = ScalarExt::parse_arr(&v).unwrap();

        Coin {
            addr_pk: Some(addr_pk),
            addr_sk: Some(addr_sk),
            rho: Some(rho),
            r: Some(r),
            s: Some(s),
            v: Some(v),
            cm: Some(cm),
            user_id: Some(user_id),
            status: Some(Status::Unused),
        }
    }
}

pub(crate) struct OwnCoin {
    pub addr_sk: Option<Scalar>,

    pub rho: Option<Scalar>,

    pub r: Option<Scalar>,

    pub s: Option<Scalar>,

    pub v: Option<Scalar>,

    pub user_id: Option<String>,

    pub status: Option<Status>,
}
