use crate::WalletError;
use sak_crypto::{
    groth16, mimc, os_rng, Bls12, Circuit, Hasher, Proof, Scalar, ScalarExt,
};
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};
use saksaha::{generate_proof_1_to_2, get_auth_path};
use std::{collections::HashMap, time::Duration};

pub struct OwnCoin {
    pub addr_sk: Option<Scalar>,

    pub rho: Option<Scalar>,

    pub r: Option<Scalar>,

    pub s: Option<Scalar>,

    pub v: Option<Scalar>,

    pub useable: Option<bool>,
}

pub(crate) async fn put_old_coin(old_coin: OldCoin) {
    // self.db.schema.put_old_coin
}

pub(crate) async fn get_old_coin(cm: String) -> OldCoin {
    // self.db.schema.get_old_coin
    OldCoin::default()
}

pub(crate) async fn generate_old_coin(
    idx: u128,
) -> Result<OldCoin, WalletError> {
    // TODO: if failed to load full auth_path, return error
    let auth_path = get_auth_path(idx).await?;
    Ok(OldCoin::default())
}

pub(crate) async fn put_new_coin(new_coin: NewCoin) {
    // self.db.schema.put_old_coin
}

pub(crate) async fn generate_new_coin(value: u64) -> NewCoin {
    NewCoin::default()
}

pub(crate) async fn generate_proof(
    old_coin: OldCoin,
    new_coin_1: NewCoin,
    new_coin_2: NewCoin,
) -> Result<Proof<Bls12>, WalletError> {
    let pi = generate_proof_1_to_2(old_coin, new_coin_1, new_coin_2).await?;

    Ok(pi)
}

// rpc routes features
// saksaha::send_tx_pour(ctr_addr, req_type, arg)
// saksaha::send_tx_mint(ctr_addr, req_type, arg)
