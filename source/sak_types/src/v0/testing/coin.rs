use core::fmt;

use sak_crypto::ScalarExt;
use sak_proofs::Hasher;
use type_extension::U8Array;

#[derive(Debug)]
pub struct MockCoin {
    pub addr_sk: [u8; 32],
    pub addr_pk: [u8; 32],
    pub rho: [u8; 32],
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub v: [u8; 32],
    pub k: [u8; 32],
    pub cm: [u8; 32],
}

impl fmt::Display for MockCoin {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "\
            \n[!] Coin Info 
[+] addr_sk: {:?}, 
[+] addr_pk: {:?}, 
[+] rho: {:?}, 
[+] r: {:?}, 
[+] s: {:?}, 
[+] v: {:?}, 
[+] k: {:?}, 
[+] cm: {:?}",
            ScalarExt::parse_arr(&self.addr_sk),
            ScalarExt::parse_arr(&self.addr_pk),
            ScalarExt::parse_arr(&self.rho),
            ScalarExt::parse_arr(&self.r),
            ScalarExt::parse_arr(&self.s),
            ScalarExt::parse_arr(&self.v),
            self.k,
            self.cm
        )?;

        Ok(())
    }
}

pub fn mock_coin(value: u64) -> MockCoin {
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

    MockCoin {
        addr_sk,
        addr_pk: addr_pk.to_bytes(),
        rho,
        r,
        s,
        v,
        k: k.to_bytes(),
        cm: cm.to_bytes(),
    }
}

pub fn mock_coin_custom(
    rho: u64,
    r: u64,
    s: u64,
    addr_sk: u64,
    value: u64,
) -> MockCoin {
    let hasher = Hasher::new();

    let addr_sk = U8Array::from_int(addr_sk).to_owned();
    let addr_pk = hasher.mimc_single(&addr_sk).unwrap();
    let rho = U8Array::from_int(rho);
    let r = U8Array::from_int(r);
    let s = U8Array::from_int(s);
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

    MockCoin {
        addr_sk,
        addr_pk: addr_pk.to_bytes(),
        rho,
        r,
        s,
        v,
        k: k.to_bytes(),
        cm: cm.to_bytes(),
    }
}
