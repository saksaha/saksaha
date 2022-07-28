use crate::{
    rpc::{
        router::{self, Params},
        RPCError,
    },
    system::SystemHandle,
};
use hyper::{Body, Request, Response, StatusCode};
use log::warn;
use sak_contract_std::Request as CtrRequest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct QueryCtrRequest {
    pub ctr_addr: String,
    pub req: CtrRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct QueryCtrResponse {
    pub result: String,
}

pub(crate) async fn query_ctr(
    res: Response<Body>,
    id: String,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let params = params.ok_or::<RPCError>("".into())?;

    let rb = router::parse_params::<QueryCtrRequest>(&params)?;

    match sys_handle
        .machine
        .blockchain
        .dist_ledger
        .apis
        .query_ctr(&rb.ctr_addr, rb.req)
        .await
    {
        Ok(t) => {
            return Ok(router::make_success_response(
                res,
                id,
                QueryCtrResponse { result: t },
            ));
        }
        Err(err) => {
            return Ok(router::make_error_response(
                res,
                Some(String::from("1")),
                err.into(),
            ));
        }
    }
}
