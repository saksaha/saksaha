use super::apis;
use crate::{
    credential::Credential, db::DB, rpc::routes::SendTxRequest, WalletError,
};
use futures::sink::Send;
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};

pub(crate) struct Wallet {
    // db
    credential: Credential,
}

impl Wallet {
    pub async fn init(
        app_prefix: String,
        credential: Credential,
    ) -> Result<Wallet, WalletError> {
        let db = DB::init(&app_prefix).await?;

        let w = Wallet { credential };

        Ok(w)
    }

    pub fn get_balance(&self) {
        // from db
        // get some data
        // return
    }

    pub async fn send_tx(
        &self,
        send_tx_req: SendTxRequest,
    ) -> Result<(), WalletError> {
        {
            let val = send_tx_req.get_value();

            // let old_coin = prepare_old_coin(val).await?;
            // saksaha::send_tx_pour(ctr_addr, req_type, arg).await?;
        }

        // from db
        // check if some data is something
        // return
        Ok(())
    }
}
