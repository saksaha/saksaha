use super::WalletApis;
use crate::WalletError;

use crate::rpc::routes::v0::WalletSendTxRequest;
use sak_crypto::{
    groth16, mimc, os_rng, Bls12, Circuit, Hasher, Proof, Scalar, ScalarExt,
};
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};
use sak_types::Balance;
use saksaha::{generate_proof_1_to_2, get_auth_path};

impl WalletApis {
    pub async fn get_balance(
        &self,
        id: String,
        key: String,
    ) -> Result<Balance, WalletError> {
        println!("wallet apis, get_balance, id: {}", id);

        &self.db.schema;

        let b = Balance { val: 0 };

        Ok(b)
    }

    pub async fn send_tx(
        &self,
        req: WalletSendTxRequest,
        // send_tx_req: SendTxRequest,
    ) -> Result<(), WalletError> {
        let WalletSendTxRequest {
            val,
            ctr_addr,
            req_type,
            args,
        } = req;

        {
            // let old_coin = prepare_old_coin(val).await?;
            saksaha::send_tx_pour(ctr_addr, req_type, args).await?;
        }

        // from db
        // check if some data is something
        // return
        Ok(())
    }

    pub(crate) async fn prepare_old_coin(
        &self,
        idx: [u8; 32],
    ) -> Result<OldCoin, WalletError> {
        //TODO :
        let idx = 0;
        let auth_path = get_auth_path(idx).await?;
        let coin = OldCoin::default();

        self.put_old_coin(&coin);

        Ok(coin)
    }

    pub(crate) async fn put_old_coin(&self, old_coin: &OldCoin) {
        // self.db.schema.put_old_coin
    }

    pub(crate) async fn get_old_coin(&self, cm: String) -> OldCoin {
        // self.db.schema.get_old_coin
        OldCoin::default()
    }

    pub(crate) async fn put_new_coin(&self, new_coin: NewCoin) {
        // self.db.schema.put_old_coin
    }

    pub(crate) async fn generate_new_coin(&self, value: u64) -> NewCoin {
        NewCoin::default()
    }

    pub(crate) async fn generate_proof(
        &self,
        old_coin: OldCoin,
        new_coin_1: NewCoin,
        new_coin_2: NewCoin,
    ) -> Result<Proof<Bls12>, WalletError> {
        let pi =
            generate_proof_1_to_2(old_coin, new_coin_1, new_coin_2).await?;

        Ok(pi)
    }

    // rpc routes features
    // saksaha::send_tx_pour(ctr_addr, req_type, arg)
    // saksaha::send_tx_mint(ctr_addr, req_type, arg)
}
