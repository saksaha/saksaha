use super::{errors, header::HeaderFactory, HandleError, HandleError2};
use crate::rpc::RPCError;
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Response, StatusCode,
};
use serde::{Deserialize, Serialize};

const JSON_RPC_VERSION: &'static str = "2.0";

#[derive(Serialize, Deserialize, Debug)]
pub(in crate::rpc) struct RPCResponse {}

#[derive(Serialize, Debug)]
pub(in crate::rpc) struct JsonResponse<'a, D: Serialize> {
    jsonrpc: &'static str,
    error: Option<HandleError2<'a>>,
    result: Option<D>,
    id: String,
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
                Err(err) => return handle_serialize_error(id, header_factory),
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

        *res.status_mut() = StatusCode::OK;

        *res.body_mut() = {
            let response: JsonResponse<()> = JsonResponse {
                jsonrpc: JSON_RPC_VERSION.into(),
                error: Some(HandleError2 {
                    code: "1",
                    desc: "a",
                }),
                result: None,
                id: id.to_string(),
            };

            let body_str = match serde_json::to_string(&response) {
                Ok(s) => s,
                Err(err) => return handle_serialize_error(id, header_factory),
            };

            Body::from(body_str)
        };

        res
    }
}

fn handle_serialize_error(
    id: String,
    header_factory: &HeaderFactory,
) -> Response<Body> {
    let mut res = Response::default();

    res.headers_mut()
        .insert(CONTENT_TYPE, header_factory.application_json.clone());

    *res.status_mut() = StatusCode::OK;

    *res.body_mut() = {
        let response: JsonResponse<()> = JsonResponse {
            jsonrpc: JSON_RPC_VERSION.into(),
            error: Some(HandleError2 {
                code: "1",
                desc: "1",
            }),
            result: None,
            id: id.to_string(),
        };

        let body_str = match serde_json::to_string(&response) {
            Ok(s) => s,
            Err(err) => return handle_serialize_error(id, header_factory),
        };

        Body::from(body_str)
    };

    res
}
