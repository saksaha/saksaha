use crate::{
    rpc::{
        router::{utils, Params},
        RPCError,
    },
    system::SystemHandle,
};
use hyper::{Body, Request, Response};
use log::warn;
use sak_types::Block;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetBlockRequest {
    pub block_hash: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct GetBlockResponse {
    pub block: Option<Block>,
}

pub(crate) async fn get_block(
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let params = params.ok_or::<RPCError>("".into())?;

    let rb: GetBlockRequest = utils::parse_params::<GetBlockRequest>(&params)?;

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .get_block(&rb.block_hash)
    {
        Ok(block) => {
            let get_block_resp = GetBlockResponse { block };

            return Ok(utils::make_success_response(
                String::from("1"),
                get_block_resp,
            ));
        }
        Err(err) => {
            return Ok(utils::make_error_response(
                Some(String::from("1")),
                err.into(),
            ));
        }
    }
}
