mod circuit_1_to_2;
mod proof;

pub use circuit_1_to_2::*;
pub use proof::*;
use sak_crypto::Scalar;

pub const CM_TREE_DEPTH: u32 = 5;
pub const CM_TREE_CAPACITY: usize = 2_usize.pow(CM_TREE_DEPTH as u32);

#[derive(Debug, Copy, Clone)]
pub struct OldCoin {
    pub addr_pk: Option<Scalar>,

    pub addr_sk: Option<Scalar>,

    pub rho: Option<Scalar>,

    pub r: Option<Scalar>,

    pub s: Option<Scalar>,

    pub v: Option<Scalar>,

    pub cm: Option<Scalar>,

    pub auth_path: [Option<(Scalar, bool)>; CM_TREE_DEPTH as usize],
}

impl OldCoin {
    pub fn default() -> Self {
        OldCoin {
            addr_pk: None,

            addr_sk: None,

            rho: None,

            r: None,

            s: None,

            v: None,

            cm: None,

            auth_path: [None; CM_TREE_DEPTH as usize],
        }
    }

    pub fn new() -> Self {
        OldCoin {
            addr_pk: None,

            addr_sk: None,

            rho: None,

            r: None,

            s: None,

            v: None,

            cm: None,

            auth_path: [None; CM_TREE_DEPTH as usize],
        }
    }

    pub fn update_auth_path(
        &mut self,
        auth_path: [Option<(Scalar, bool)>; CM_TREE_DEPTH as usize],
    ) {
        self.auth_path = auth_path;
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NewCoin {
    pub addr_pk: Option<Scalar>,

    pub rho: Option<Scalar>,

    pub r: Option<Scalar>,

    pub s: Option<Scalar>,

    pub v: Option<Scalar>,
}

impl NewCoin {
    pub fn default() -> Self {
        NewCoin {
            addr_pk: None,

            rho: None,

            r: None,

            s: None,

            v: None,
        }
    }
}
