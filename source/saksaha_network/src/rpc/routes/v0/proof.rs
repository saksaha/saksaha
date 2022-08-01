use crate::{
    rpc::{
        router::{utils, Params},
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
pub(crate) struct AuthPathRequest {
    pub location: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AuthPathResponse {
    pub result: Vec<Option<[u8; 32]>>,
}

pub(crate) async fn get_auth_path(
    id: String,
    params: Params,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, RPCError> {
    let params = params.ok_or::<RPCError>("".into())?;

    let rb = utils::parse_params::<AuthPathRequest>(&params)?;

    let locations = rb.location;

    let mut auth_path = Vec::new();

    for loc in locations {
        match sys_handle
            .machine
            .blockchain
            .dist_ledger
            .apis
            .get_merkle_node(&loc)
            .await
        {
            Ok(n) => {
                auth_path.push(n);
            }
            Err(err) => {
                return Ok(utils::make_error_response(
                    Some(String::from("1")),
                    err.into(),
                ));
            }
        }
    }

    Ok(utils::make_success_response(
        id,
        AuthPathResponse { result: auth_path },
    ))
}
