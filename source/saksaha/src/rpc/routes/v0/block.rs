use crate::{
    rpc::{RPCError, RPCResponse},
    system::SystemHandle,
};
use hyper::{Body, Request, Response};
use log::warn;
use sak_types::Block;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub(in crate::rpc) struct GetBlockRequest {
    pub block_hash: String,
}

#[derive(Serialize)]
pub(in crate::rpc) struct GetBlockResponse {
    pub block: Option<Block>,
}

pub(crate) async fn get_block(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let b = hyper::body::to_bytes(req.into_body()).await?;

    let rb = serde_json::from_slice::<GetBlockRequest>(&b)?;

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .get_block(&rb.block_hash)
    {
        Ok(block) => {
            let get_block_resp = GetBlockResponse { block };

            return Ok(RPCResponse::new_success(
                String::from("1"),
                get_block_resp,
            ));
        }
        Err(err) => {
            return Ok(RPCResponse::new_error(String::from("1"), err.into()));
        }
    }
}
