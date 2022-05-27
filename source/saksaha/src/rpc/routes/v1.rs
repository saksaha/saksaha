use super::Node;
use crate::blockchain::blockchain::{Hash, TxValue};
use crate::rpc::response::{ErrorResult, SuccessResult};
use hyper::{Body, Request, Response, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub(crate) async fn send_transaction(
    req: Request<Body>,
    node: Arc<Node>,
) -> Result<Response<Body>, hyper::Error> {
    let _body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body_bytes_vec = b.to_vec();
            let _body_str = match std::str::from_utf8(&body_bytes_vec) {
                Ok(b) => {
                    let _tx_value: TxValue = match serde_json::from_str(b) {
                        Ok(v) => match node.machine.send_transaction(v).await {
                            Ok(hash) => {
                                return SuccessResult {
                                    id: String::from("1"),
                                    result: hash,
                                }
                                .into_hyper_result();
                            }
                            Err(err) => {
                                return ErrorResult::<String> {
                                    id: String::from("1"),
                                    status_code: StatusCode::BAD_REQUEST,
                                    code: 32600,
                                    message: String::from("Invalid Request"),
                                    data: None,
                                }
                                .into_hyper_result();
                            }
                        },
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

pub(crate) async fn get_transaction(
    req: Request<Body>,
    node: Arc<Node>,
) -> Result<Response<Body>, hyper::Error> {
    let _body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body_bytes_vec = b.to_vec();
            let _body_str = match std::str::from_utf8(&body_bytes_vec) {
                Ok(b) => {
                    println!("{}", b);
                    let _tx: Hash = match serde_json::from_str(b) {
                        Ok(tx_hash) => {
                            match node.machine.get_transaction(tx_hash).await {
                                Ok(t) => {
                                    return SuccessResult {
                                        id: String::from("1"),
                                        result: t,
                                    }
                                    .into_hyper_result();
                                }
                                Err(err) => {
                                    return ErrorResult::<String> {
                                        id: String::from("1"),
                                        status_code: StatusCode::BAD_REQUEST,
                                        code: 32600,
                                        message: String::from(
                                            "Invalid Request",
                                        ),
                                        data: Some(err.to_string()),
                                    }
                                    .into_hyper_result();
                                }
                            }
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
    node: Arc<Node>,
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

    let addr_vec = node.p2p_monitor.p2p_discovery.addr_table.get_status().await;
    let peer_vec = node.p2p_monitor.peer_table.get_status().await;

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
    node: Arc<Node>,
) -> Result<Response<Body>, hyper::Error> {
    match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body_bytes_vec = b.to_vec();
            match std::str::from_utf8(&body_bytes_vec) {
                Ok(b) => {
                    let hash = &Hash {
                        hash: b.to_string(),
                    };
                    match node.machine.get_block(hash).await {
                        Ok(block) => {
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
