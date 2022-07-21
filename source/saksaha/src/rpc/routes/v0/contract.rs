use crate::{
    rpc::{RPCError, RPCResponse},
    system::SystemHandle,
};
use hyper::{Body, Request, Response, StatusCode};
use log::warn;
use sak_contract_std::Request as CtrRequest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CallContractBody {
    pub ctr_addr: String,
    pub req: CtrRequest,
}

pub(crate) async fn call_contract(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let b = hyper::body::to_bytes(req.into_body()).await?;

    let rb = serde_json::from_slice::<CallContractBody>(&b)?;

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .query_ctr(&rb.ctr_addr, rb.req)
        .await
    {
        Ok(t) => {
            return Ok(RPCResponse::new_success(String::from("1"), t));
        }
        Err(err) => {
            return Ok(RPCResponse::new_error(String::from("1"), err.into()));
        }
    }
}
