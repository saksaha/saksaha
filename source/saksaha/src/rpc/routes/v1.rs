use crate::rpc::response::{ErrorResult, SuccessResult};
use crate::{blockchain::blockchain::TxValue, machine::Machine};
use hyper::{Body, Request, Response, StatusCode};
use std::sync::Arc;

pub(crate) async fn send_transaction(
    req: Request<Body>,
    machine: Arc<Machine>,
) -> Result<Response<Body>, hyper::Error> {
    let _body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body_bytes_vec = b.to_vec();
            let _body_str = match std::str::from_utf8(&body_bytes_vec) {
                Ok(b) => {
                    let _tx_value: TxValue = match serde_json::from_str(b) {
                        Ok(v) => match machine.send_transaction(v).await {
                            Ok(h) => {
                                let mut response = Response::default();
                                *response.body_mut() = Body::from(h);
                                println!("response: {:?}", &response);
                                return Ok(response);
                            }
                            Err(err) => {
                                println!("err: {}", err);
                                return Ok(incorrect_tx_hash_response());
                            }
                        },
                        Err(err) => {
                            return ErrorResult {
                                code: 1414,
                                message: String::from("dummy"),
                                data: err.to_string(),
                            }
                            .into_hyper_result();
                        }
                    };
                }
                Err(err) => {
                    return ErrorResult {
                        code: 1414,
                        message: String::from("dummy"),
                        data: err.to_string(),
                    }
                    .into_hyper_result();
                }
            };
        }
        Err(err) => {
            return ErrorResult {
                code: 1414,
                message: String::from("dummy"),
                data: err.to_string(),
            }
            .into_hyper_result();
        }
    };
}

pub(crate) async fn dummy(
    req: Request<Body>,
    machine: Arc<Machine>,
) -> Result<Response<Body>, hyper::Error> {
    let body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body_bytes_vec = b.to_vec();
            let body_str = match std::str::from_utf8(&body_bytes_vec) {
                Ok(b) => {
                    let tx_value: TxValue = match serde_json::from_str(b) {
                        Ok(v) => v,
                        Err(err) => {
                            return ErrorResult {
                                code: 1414,
                                message: String::from("dummy"),
                                data: err.to_string(),
                            }
                            .into_hyper_result();
                        }
                    };
                }
                Err(err) => {
                    return ErrorResult {
                        code: 1414,
                        message: String::from("dummy"),
                        data: err.to_string(),
                    }
                    .into_hyper_result();
                }
            };
        }
        Err(err) => {
            return ErrorResult {
                code: 1414,
                message: String::from("dummy"),
                data: err.to_string(),
            }
            .into_hyper_result();
        }
    };

    return SuccessResult {
        result: String::from("power"),
    }
    .into_hyper_result();
}

pub fn incorrect_tx_hash_response() -> Response<Body> {
    let mut no_content = Response::default();
    *no_content.status_mut() = StatusCode::NO_CONTENT;

    no_content
}
