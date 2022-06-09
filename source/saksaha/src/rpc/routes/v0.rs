use crate::rpc::response::{ErrorResult, SuccessResult};
use crate::system::SystemHandle;
use hyper::{Body, Request, Response, StatusCode};
use log::warn;
use sak_types::Transaction;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub(crate) async fn send_transaction(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, hyper::Error> {
    let _body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body_bytes_vec = b.to_vec();
            let _body_str = match std::str::from_utf8(&body_bytes_vec) {
                Ok(b) => {
                    let _tx_value: Transaction = match serde_json::from_str(b) {
                        Ok(v) => {
                            match sys_handle.machine.send_transaction(v).await {
                                Ok(bool) => {
                                    return SuccessResult {
                                        id: String::from("1"),
                                        result: bool,
                                    }
                                    .into_hyper_result();
                                }
                                Err(err) => {
                                    return ErrorResult::<String> {
                                        id: String::from("1"),
                                        status_code: StatusCode::BAD_REQUEST,
                                        code: 32600,
                                        message: String::from(err),
                                        data: None,
                                    }
                                    .into_hyper_result();
                                }
                            }
                        }
                        Err(err) => {
                            warn!("Error parsing request param, err: {}", err);

                            return ErrorResult {
                                id: String::from("1"),
                                status_code: StatusCode::BAD_REQUEST,
                                code: 32601,
                                message: String::from("Invalid Request"),
                                data: Some(err.to_string()),
                            }
                            .into_hyper_result();
                        }
                    };
                }
                Err(err) => {
                    return ErrorResult {
                        id: String::from("1"),
                        status_code: StatusCode::BAD_REQUEST,
                        code: 32602,
                        message: String::from("Invalid Request"),
                        data: Some(err.to_string()),
                    }
                    .into_hyper_result();
                }
            };
        }
        Err(err) => {
            return ErrorResult {
                id: String::from("1"),
                status_code: StatusCode::BAD_REQUEST,
                code: 32603,
                message: String::from("Invalid Request"),
                data: Some(err.to_string()),
            }
            .into_hyper_result();
        }
    };
}

#[derive(Deserialize, Debug)]
struct GetTransactionBody {
    hash: String,
}

pub(crate) async fn get_transaction(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, hyper::Error> {
    match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body: GetTransactionBody = match serde_json::from_slice(&b) {
                Ok(b) => GetTransactionBody { hash: b },
                Err(err) => {
                    return ErrorResult::<String> {
                        id: String::from("1"),
                        status_code: StatusCode::BAD_REQUEST,
                        code: 32600,
                        message: String::from("Invalid Request"),
                        data: Some(err.to_string()),
                    }
                    .into_hyper_result();
                }
            };

            match sys_handle.machine.get_transaction(body.hash).await {
                Ok(t) => {
                    println!("apowef: {:?}", t);

                    return SuccessResult {
                        id: String::from("1"),
                        result: t,
                    }
                    .into_hyper_result();
                }
                Err(err) => {
                    println!("apowef333: {:?}", err);
                    return ErrorResult::<String> {
                        id: String::from("1"),
                        status_code: StatusCode::BAD_REQUEST,
                        code: 32600,
                        message: String::from("Invalid Request"),
                        data: Some(err.to_string()),
                    }
                    .into_hyper_result();
                }
            }

            // let _body_str = match std::str::from_utf8(&body_bytes_vec) {
            //     Ok(b) => {
            //         println!("awefla, b: {}", b);
            //         let aa = &format!(
            //             r#"
            //                 {{"hash": "{}"}}
            //             "#,
            //             1
            //         );
            //         println!("aa: {}", aa);

            //         let x: GetTransactionBody =
            //             serde_json::from_str(aa).unwrap();

            //         let x2: GetTransactionBody =
            //             serde_json::from_str(b).unwrap();

            //         println!("x: {:?}, x2: {:?}", x, x2);

            //         let _tx_hash: GetTransactionBody =
            //             match serde_json::from_str(b) {
            //                 Ok(tx_hash) => {
            //                     println!("tx_hash: {}", tx_hash);

            //                     match sys_handle
            //                         .machine
            //                         .get_transaction(tx_hash.hash)
            //                         .await
            //                     {
            //                         Ok(t) => {
            //                             println!("apowef: {:?}", t);

            //                             return SuccessResult {
            //                                 id: String::from("1"),
            //                                 result: t,
            //                             }
            //                             .into_hyper_result();
            //                         }
            //                         Err(err) => {
            //                             println!("apowef333: {:?}", err);
            //                             return ErrorResult::<String> {
            //                                 id: String::from("1"),
            //                                 status_code:
            //                                     StatusCode::BAD_REQUEST,
            //                                 code: 32600,
            //                                 message: String::from(
            //                                     "Invalid Request",
            //                                 ),
            //                                 data: Some(err.to_string()),
            //                             }
            //                             .into_hyper_result();
            //                         }
            //                     }
            //                 }
            //                 Err(err) => {
            //                     println!("awefla, err: {}", err);
            //                     return ErrorResult {
            //                         id: String::from("1"),
            //                         status_code: StatusCode::BAD_REQUEST,
            //                         code: 32600,
            //                         message: String::from("Invalid Request"),
            //                         data: Some(err.to_string()),
            //                     }
            //                     .into_hyper_result();
            //                 }
            //             };
            //     }
            //     Err(err) => {
            //         return ErrorResult {
            //             id: String::from("1"),
            //             status_code: StatusCode::BAD_REQUEST,
            //             code: 32600,
            //             message: String::from("Invalid Request"),
            //             data: Some(err.to_string()),
            //         }
            //         .into_hyper_result();
            //     }
            // };
        }
        Err(err) => {
            return ErrorResult {
                id: String::from("1"),
                status_code: StatusCode::BAD_REQUEST,
                code: 32600,
                message: String::from("Invalid Request"),
                data: Some(err.to_string()),
            }
            .into_hyper_result();
        }
    };
}

#[derive(Serialize, Deserialize, Debug)]
struct NodeStatus {
    addr_vec: Vec<String>,
    peer_vec: Vec<String>,
}

pub(crate) async fn get_status(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, hyper::Error> {
    let _body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body_bytes_vec = b.to_vec();
            let _body_str = match std::str::from_utf8(&body_bytes_vec) {
                Ok(_) => {}
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

    let addr_vec = sys_handle
        .p2p_monitor
        .p2p_discovery
        .addr_table
        .get_status()
        .await;
    let peer_vec = sys_handle.p2p_monitor.peer_table.get_status().await;

    let result = NodeStatus {
        addr_vec, //
        peer_vec,
    };

    return SuccessResult {
        id: String::from("1"),
        result,
    }
    .into_hyper_result();
}

pub(crate) async fn get_block(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, hyper::Error> {
    match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body_bytes_vec = b.to_vec();
            match std::str::from_utf8(&body_bytes_vec) {
                Ok(b) => {
                    let block_hash = b.to_string();

                    match sys_handle.machine.get_block(&block_hash).await {
                        Ok(_block) => {
                            return SuccessResult {
                                id: String::from("1"),
                                result: String::from(""),
                            }
                            .into_hyper_result()
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
                    }
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
