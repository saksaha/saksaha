use crate::CircuitError;
use sak_crypto::{hasher::MiMC, rand, Scalar, ScalarExt};
use sak_dist_ledger_cfg::CM_TREE_DEPTH;

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

    pub fn new_dummy() -> Result<OldCoin, CircuitError> {
        let hasher = MiMC::new();

        let addr_sk = ScalarExt::parse_u64(0)?;
        let addr_pk = hasher.mimc_single_scalar(addr_sk)?;
        let rho = ScalarExt::parse_u64(0)?;
        let r = ScalarExt::parse_u64(0)?;
        let s = ScalarExt::parse_u64(0)?;
        let v = ScalarExt::parse_u64(0)?;

        // sn : 0x46205869c121af666efa3ca1114c4f01837f407ca0b2c97c0ecaa957e5836bd6

        // let addr_sk = ScalarExt::parse_u64(rand() as u64)?;
        // let addr_pk = hasher.mimc_single_scalar(addr_sk)?;
        // let rho = ScalarExt::parse_u64(rand() as u64)?;
        // let r = ScalarExt::parse_u64(rand() as u64)?;
        // let s = ScalarExt::parse_u64(rand() as u64)?;
        // let v = ScalarExt::parse_u64(rand() as u64)?;

        let k = hasher.comm2_scalar(r, addr_pk, rho);
        let cm = hasher.comm2_scalar(s, v, k);

        Ok(OldCoin {
            addr_pk: Some(addr_pk),
            addr_sk: Some(addr_sk),
            rho: Some(rho),
            r: Some(r),
            s: Some(s),
            v: Some(v),
            cm: Some(cm),
            auth_path: [None; CM_TREE_DEPTH as usize],
        })
    }

    pub fn update_auth_path(
        &mut self,
        auth_path: [Option<(Scalar, bool)>; CM_TREE_DEPTH as usize],
    ) {
        self.auth_path = auth_path;
    }

    pub fn compute_sn(&self) -> Result<Scalar, CircuitError> {
        let sn = {
            let addr_sk = self.addr_sk.ok_or("Failed to get addr_sk")?;

            let rho = self.rho.ok_or("Failed to get rho")?;

            let hasher = MiMC::new();

            let s = hasher.mimc_scalar(addr_sk, rho);

            s
        };

        Ok(sn)
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

    pub fn compute_cm(&self) -> Result<Scalar, CircuitError> {
        {
            if self.r.is_none()
                || self.s.is_none()
                || self.v.is_none()
                || self.rho.is_none()
                || self.addr_pk.is_none()
            {
                return Err(format!("NewCoin has insufficient arguments for computing cm").into());
            }
        }

        let hasher = MiMC::new();

        let k = hasher.comm2_scalar(self.r.unwrap(), self.addr_pk.unwrap(), self.rho.unwrap());

        let cm = hasher.comm2_scalar(self.s.unwrap(), self.v.unwrap(), k);

        Ok(cm)
    }
}
