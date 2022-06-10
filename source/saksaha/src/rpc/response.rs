use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Response, StatusCode,
};
use serde::{Deserialize, Serialize};

const JSON_RPC: &str = "2.0";

#[derive(Debug, Serialize)]
pub(crate) struct SuccessResult<D: Serialize> {
    pub(crate) id: String,
    pub(crate) result: D,
}

pub(crate) struct ErrorResult<E: Serialize> {
    pub(crate) status_code: StatusCode,
    pub(crate) id: String,
    pub(crate) code: usize,
    pub(crate) message: String,
    pub(crate) data: Option<E>,
}

impl<D: Serialize> SuccessResult<D> {
    pub(crate) fn into_hyper_result(
        &self,
    ) -> Result<Response<Body>, hyper::Error> {
        let mut res = Response::default();
        *res.status_mut() = StatusCode::OK;

        *res.body_mut() = {
            let result = serde_json::to_string(&self.result).unwrap();

            let response = SuccessResponse {
                jsonrpc: JSON_RPC.into(),
                result,
                id: self.id.clone(),
            };

            let body_str = serde_json::to_string(&response).unwrap();

            Body::from(body_str)
        };

        Ok(res)
    }
}

impl<E: Serialize> ErrorResult<E> {
    pub fn into_hyper_result(&self) -> Result<Response<Body>, hyper::Error> {
        let mut res = Response::default();

        let headers = res.headers_mut();
        headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/json").unwrap(),
        );

        *res.status_mut() = StatusCode::OK;

        *res.body_mut() = {
            let data = serde_json::to_string(&self.data).unwrap();

            let response = JsonResponse {
                jsonrpc: JSON_RPC.into(),
                result: None,
                error: Some(Error {
                    code: self.code,
                    message: self.message.clone(),
                    data,
                }),
                id: self.id.clone(),
            };

            let body_str = serde_json::to_string(&response).unwrap();

            Body::from(body_str)
        };

        Ok(res)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct SuccessResponse {
    jsonrpc: String,
    result: String,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct JsonResponse {
    jsonrpc: String,
    error: Option<Error>,
    result: Option<String>,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct ErrorResponse {
    jsonrpc: String,
    result: Option<String>,
    error: Error,
    id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(super) struct Error {
    code: usize,
    message: String,
    data: String,
}
