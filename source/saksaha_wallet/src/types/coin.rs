use super::Status;
use sak_crypto::Scalar;

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
