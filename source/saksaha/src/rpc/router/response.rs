use super::utils;
use super::HandleError;
use crate::rpc::{router::HeaderFactory, RPCError};
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Response, StatusCode,
};
use serde::{Deserialize, Serialize};

pub(in crate::rpc) const JSON_RPC_VERSION: &'static str = "2.0";

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct RPCResponse {}

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct JsonResponse<D: Serialize> {
    pub jsonrpc: String,
    pub error: Option<HandleError>,
    pub result: Option<D>,
    pub id: String,
}

impl RPCResponse {
    pub fn new_success<D: Serialize>(id: String, result: D) -> Response<Body> {
        let header_factory = HeaderFactory::get_instance().expect(
            "Header factory should have \
            been initialized.",
        );

        let mut res = Response::default();

        res.headers_mut()
            .insert(CONTENT_TYPE, header_factory.application_json.clone());

        *res.status_mut() = StatusCode::OK;

        *res.body_mut() = {
            let response = JsonResponse {
                jsonrpc: JSON_RPC_VERSION.into(),
                error: None,
                result: Some(result),
                id: id.to_string(),
            };

            let body_str = match serde_json::to_string(&response) {
                Ok(s) => s,
                Err(err) => {
                    return utils::make_serialize_err_response(
                        id,
                        Some(err.into()),
                    )
                }
            };

            Body::from(body_str)
        };

        res
    }

    pub fn new_error(id: String, error: RPCError) -> Response<Body> {
        let header_factory = HeaderFactory::get_instance().expect(
            "Header factory should have \
            been initialized.",
        );

        let mut res = Response::default();

        res.headers_mut()
            .insert(CONTENT_TYPE, header_factory.application_json.clone());

        *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

        *res.body_mut() = {
            let response: JsonResponse<()> = JsonResponse {
                jsonrpc: JSON_RPC_VERSION.into(),
                error: Some(HandleError {
                    msg: error.to_string(),
                }),
                result: None,
                id: id.to_string(),
            };

            let body_str = match serde_json::to_string(&response) {
                Ok(s) => s,
                Err(err) => {
                    return utils::make_serialize_err_response(
                        id,
                        Some(err.into()),
                    );
                }
            };

            Body::from(body_str)
        };

        res
    }
}
