use crate::{
    rpc::{RPCError, RPCResponse},
    system::SystemHandle,
};
use hyper::{Body, Request, Response};
use log::warn;
use std::sync::Arc;

pub(crate) async fn get_block(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let b = hyper::body::to_bytes(req.into_body()).await?;

    let rb = std::str::from_utf8(&b.to_vec())?.to_string();

    let block_hash = rb;

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .get_block(&block_hash)
    {
        Ok(_block) => {
            return Ok(RPCResponse::new_success(String::from("1"), "1"));
        }
        Err(err) => {
            return Ok(RPCResponse::new_error(String::from("1"), err.into()));
        }
    }
}
