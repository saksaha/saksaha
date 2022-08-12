use sak_crypto::Scalar;
use sak_proofs::CM_TREE_DEPTH;

pub struct CoinRecord {
    pub addr_pk: Scalar,

    pub addr_sk: Scalar,

    pub rho: Scalar,

    pub r: Scalar,

    pub s: Scalar,

    pub v: Scalar,

    pub cm: Scalar,

    pub auth_path: [(Scalar, bool); CM_TREE_DEPTH as usize],

    pub merkle_rt: Scalar,
}

impl CoinRecord {
    pub fn a() {}
}
