use crate::rpc::response::{ErrorResult, SuccessResult};
use crate::{blockchain::blockchain::TxValue, machine::Machine};
use hyper::{Body, Request, Response, StatusCode};
use p2p_discovery::Discovery;
use std::convert::TryInto;
use std::{str::Utf8Error, sync::Arc};

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
                            Ok(hash) => {
                                println!("response: {:?}", &hash);

                                return SuccessResult {
                                    id: String::from("1"),
                                    result: hash,
                                }
                                .into_hyper_result();
                            }
                            Err(err) => {
                                println!("err: {}", err);

                                return ErrorResult::<String> {
                                    id: String::from("1"),
                                    status_code: StatusCode::NO_CONTENT,
                                    code: 1414,
                                    message: String::from("dummy"),
                                    data: None,
                                }
                                .into_hyper_result();
                            }
                        },
                        Err(err) => {
                            return ErrorResult {
                                id: String::from("1"),
                                status_code: StatusCode::NO_CONTENT,
                                code: 1414,
                                message: String::from("dummy"),
                                data: Some(err.to_string()),
                            }
                            .into_hyper_result();
                        }
                    };
                }
                Err(err) => {
                    return ErrorResult {
                        id: String::from("1"),
                        status_code: StatusCode::NO_CONTENT,
                        code: 1414,
                        message: String::from("dummy"),
                        data: Some(err.to_string()),
                    }
                    .into_hyper_result();
                }
            };
        }
        Err(err) => {
            return ErrorResult {
                id: String::from("1"),
                status_code: StatusCode::NO_CONTENT,
                code: 1414,
                message: String::from("dummy"),
                data: Some(err.to_string()),
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
                                id: String::from("1"),
                                status_code: StatusCode::NO_CONTENT,
                                code: 1414,
                                message: String::from("dummy"),
                                data: Some(err.to_string()),
                            }
                            .into_hyper_result();
                        }
                    };
                }
                Err(err) => {
                    return ErrorResult {
                        id: String::from("1"),
                        status_code: StatusCode::NO_CONTENT,
                        code: 1414,
                        message: String::from("dummy"),
                        data: Some(err.to_string()),
                    }
                    .into_hyper_result();
                }
            };
        }
        Err(err) => {
            return ErrorResult {
                id: String::from("1"),
                status_code: StatusCode::NO_CONTENT,
                code: 1414,
                message: String::from("dummy"),
                data: Some(err.to_string()),
            }
            .into_hyper_result();
        }
    };

    return SuccessResult {
        id: String::from("1"),
        result: String::from("power"),
    }
    .into_hyper_result();
}

pub(crate) async fn get_status(
    req: Request<Body>,
    machine: Arc<Machine>,
) -> Result<Response<Body>, hyper::Error> {
    let addr_vec = machine.get_status().await;
    // println!("addr_vec: {:?}", addr_vec);

    return SuccessResult {
        id: String::from("1"),
        result: String::from("power"),
    }
    .into_hyper_result();
}
