use hyper::{Body, Response, StatusCode};
use serde::Serialize;

const JSON_RPC: &str = "2.0";

#[derive(Debug, Serialize)]
pub(crate) struct SuccessResult<D: Serialize> {
    pub(crate) result: D,
}

pub(crate) struct ErrorResult<E: Serialize> {
    pub(crate) code: usize,
    pub(crate) message: String,
    pub(crate) data: E,
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
                jsonrpc: JSON_RPC,
                result,
                id: "1".into(),
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

        *res.status_mut() = StatusCode::OK;

        *res.body_mut() = {
            let data = serde_json::to_string(&self.data).unwrap();

            let response = ErrorResponse {
                jsonrpc: JSON_RPC,
                error: Error {
                    code: self.code,
                    message: self.message.clone(),
                    data,
                },
                id: "1".into(),
            };

            let body_str = serde_json::to_string(&response).unwrap();

            Body::from(body_str)
        };

        Ok(res)
    }
}

#[derive(Serialize, Debug)]
struct SuccessResponse {
    jsonrpc: &'static str,
    result: String,
    id: String,
}

#[derive(Serialize, Debug)]
struct ErrorResponse {
    jsonrpc: &'static str,
    error: Error,
    id: String,
}

#[derive(Serialize, Debug)]
struct Error {
    code: usize,
    message: String,
    data: String,
}
