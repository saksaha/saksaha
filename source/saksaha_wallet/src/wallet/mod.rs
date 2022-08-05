mod coin;

use crate::{rpc::routes::SendTxRequest, WalletError};
use coin::*;
use futures::sink::Send;
use sak_proofs::{MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};

pub(crate) struct Wallet {
    // db
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet {}
    }

    pub fn get_balance() {
        // from db
        // get some data
        // return
    }

    pub async fn send_tx(
        send_tx_req: SendTxRequest,
    ) -> Result<(), WalletError> {
        {
            let val = send_tx_req.get_value();

            let old_coin = prepare_old_coin(val).await?;
            // saksaha::send_tx_pour(ctr_addr, req_type, arg).await?;
        }

        // from db
        // check if some data is something
        // return
        Ok(())
    }
}
