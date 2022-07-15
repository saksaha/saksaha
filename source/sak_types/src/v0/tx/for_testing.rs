use super::TxCandidate;
use crate::{MintTxCandidate, PourTxCandidate, Tx, U8Array};
use sak_crypto::Hasher;

fn get_sk_1() {}

fn get_sk_2() {}

fn get_sk_3() {}

fn get_s_1() -> [u8; 32] {
    U8Array::new_empty_32()
}

fn get_s_2() -> [u8; 32] {
    U8Array::new_empty_32()
}

fn get_s_3() -> [u8; 32] {
    U8Array::new_empty_32()
}

fn get_r_1() -> [u8; 32] {
    U8Array::new_empty_32()
}

fn get_r_2() -> [u8; 32] {
    U8Array::new_empty_32()
}

fn get_r_3() -> [u8; 32] {
    U8Array::new_empty_32()
}

fn get_rho_1() -> [u8; 32] {
    U8Array::new_empty_32()
}

fn get_rho_2() -> [u8; 32] {
    U8Array::new_empty_32()
}

fn get_rho_3() -> [u8; 32] {
    U8Array::new_empty_32()
}

fn get_rho_4() -> [u8; 32] {
    U8Array::new_empty_32()
}

impl Tx {
    pub fn new_dummy_pour_1_2_3() -> Tx {
        let c = TxCandidate::new_dummy_pour_1_2_3();
        c.upgrade(0)
    }

    pub fn new_dummy_pour_2() -> Tx {
        let c = TxCandidate::new_dummy_pour_2();

        c.upgrade(1)
    }

    pub fn new_dummy_pour_3() -> Tx {
        let c = TxCandidate::new_dummy_pour_3();

        c.upgrade(2)
    }

    pub fn new_dummy_pour_4() -> Tx {
        let c = TxCandidate::new_dummy_pour_4();

        c.upgrade(3)
    }
}

impl TxCandidate {
    pub fn new_dummy_mint_1() -> TxCandidate {
        let tx_candidate = MintTxCandidate::new_dummy_1();

        TxCandidate::Mint(tx_candidate)
    }

    pub fn new_dummy_mint_2() -> TxCandidate {
        let tx_candidate = MintTxCandidate::new_dummy_2();

        TxCandidate::Mint(tx_candidate)
    }

    pub fn new_dummy_pour_1_2_3() -> TxCandidate {
        let tx_candidate = PourTxCandidate::new_dummy_1_2_3();

        TxCandidate::Pour(tx_candidate)
    }

    pub fn new_dummy_pour_2() -> TxCandidate {
        let tx_candidate = PourTxCandidate::new_dummy_2();

        TxCandidate::Pour(tx_candidate)
    }

    pub fn new_dummy_pour_3() -> TxCandidate {
        let tx_candidate = PourTxCandidate::new_dummy_3();

        TxCandidate::Pour(tx_candidate)
    }

    pub fn new_dummy_pour_4() -> TxCandidate {
        let tx_candidate = PourTxCandidate::new_dummy_4();

        TxCandidate::Pour(tx_candidate)
    }
}

impl MintTxCandidate {
    pub fn new_dummy_1() -> MintTxCandidate {
        let hasher = Hasher::new();

        let v = U8Array::from_int(1_000);

        let s = get_s_1();

        let r = get_r_1();

        let rho = get_rho_1();

        let a_pk = U8Array::new_empty_32();

        let k = hasher.comm2(&r, &a_pk, &rho).unwrap();

        let cm = hasher.comm2(&s, &v, &k.to_bytes()).unwrap();

        MintTxCandidate::new(
            String::from("created_at_mint_1"),
            vec![1],
            String::from("author_sig_mint_1"),
            None,
            cm.to_bytes(),
            v,
            k.to_bytes(),
            s,
        )
    }

    pub fn new_dummy_2() -> MintTxCandidate {
        let hasher = Hasher::new();

        let v = U8Array::from_int(1_000);

        let s = U8Array::new_empty_32();

        let r = U8Array::new_empty_32();

        let rho = U8Array::new_empty_32();

        let a_pk = U8Array::new_empty_32();

        let k = hasher.comm2(&r, &a_pk, &rho).unwrap();

        let cm = hasher.comm2(&s, &v, &k.to_bytes()).unwrap();

        MintTxCandidate::new(
            String::from("created_at_mint_1"),
            vec![1],
            String::from("author_sig_mint_1"),
            None,
            cm.to_bytes(),
            v,
            k.to_bytes(),
            s,
        )
    }
}

impl PourTxCandidate {
    pub fn new_dummy_1_2_3() -> PourTxCandidate {
        let hasher = Hasher::new();

        let pi = vec![0];

        // let sn_1 = { hasher::mimc(a_sk, rho_old) };

        let cm_1 = {
            let v = U8Array::new_empty_32();

            let s = U8Array::new_empty_32();

            let r = U8Array::new_empty_32();

            let rho = U8Array::new_empty_32();

            let a_pk = U8Array::new_empty_32();

            let k = hasher.comm2(&r, &a_pk, &rho).unwrap();

            let cm = hasher.comm2(&s, &v, &k.to_bytes());

            cm
        };

        // let ptc = PourTxCandidate::new(
        //     String::from("initial_mint_created_at"),
        //     vec![0],
        //     VALIDATOR_SIG.to_string(),
        //     None,
        //     pi,
        //     sn_1,
        //     sn_2,
        //     cm_1,
        //     cm_1,
        //     merkle_rt,
        // );

        PourTxCandidate::new(
            String::from("created_at_1"),
            vec![11, 11, 11],
            String::from("author_sig_1"),
            Some(String::from("ctr_addr_1")),
            vec![11, 11, 11],
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
        )
    }

    pub fn new_dummy_2() -> PourTxCandidate {
        PourTxCandidate::new(
            String::from("created_at_2"),
            vec![22, 22, 22],
            String::from("author_sig_2"),
            Some(String::from("ctr_addr_2")),
            vec![22, 22, 22],
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
        )
    }

    pub fn new_dummy_3() -> PourTxCandidate {
        PourTxCandidate::new(
            String::from("created_at_3"),
            vec![33, 33, 33],
            String::from("author_sig_3"),
            Some(String::from("ctr_addr_3")),
            vec![22, 22, 22],
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
        )
    }

    pub fn new_dummy_4() -> PourTxCandidate {
        PourTxCandidate::new(
            String::from("created_at_4"),
            vec![44, 44, 44],
            String::from("author_sig_4"),
            Some(String::from("ctr_addr_4")),
            vec![44, 44, 44],
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
            U8Array::new_empty_32(),
        )
    }
}
