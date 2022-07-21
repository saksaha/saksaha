use crate::rpc::response::{ErrorResult, SuccessResult};
use crate::system::SystemHandle;
use hyper::{Body, Request, Response, StatusCode};
use log::warn;
use sak_contract_std::Request as CtrRequest;
use sak_types::{MintTxCandidate, PourTxCandidate, Tx, TxCandidate};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize, Debug)]
struct SendMintTxBody {
    created_at: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    author_sig: String,
    ctr_addr: Option<String>,
    cm: [u8; 32],
    v: [u8; 32],
    k: [u8; 32],
    s: [u8; 32],
}

#[derive(Deserialize, Debug)]
struct SendPourTxBody {
    created_at: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    author_sig: String,
    ctr_addr: Option<String>,
    #[serde(with = "serde_bytes")]
    pi: Vec<u8>,
    sn_1: [u8; 32],
    sn_2: [u8; 32],
    cm_1: [u8; 32],
    cm_2: [u8; 32],
    merkle_rt: [u8; 32],
}

pub(crate) async fn send_mint_tx(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, hyper::Error> {
    match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let _tx_value: TxCandidate =
                match serde_json::from_slice::<SendMintTxBody>(&b) {
                    Ok(v) => {
                        let tx_candidate =
                            TxCandidate::Mint(MintTxCandidate::new(
                                v.created_at,
                                v.data,
                                v.author_sig,
                                v.ctr_addr,
                                v.cm,
                                v.v,
                                v.k,
                                v.s,
                            ));

                        match sys_handle
                            .machine
                            .blockchain
                            .dist_ledger
                            .apis
                            .send_tx(tx_candidate)
                            .await
                        {
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
                                    message: String::from(err.to_string()),
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
                code: 32603,
                message: String::from("Invalid Request"),
                data: Some(err.to_string()),
            }
            .into_hyper_result();
        }
    };
}

pub(crate) async fn send_pour_tx(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, hyper::Error> {
    match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let _tx_value: TxCandidate =
                match serde_json::from_slice::<SendPourTxBody>(&b) {
                    Ok(v) => {
                        let tx_candidate =
                            TxCandidate::Pour(PourTxCandidate::new(
                                v.created_at,
                                v.data,
                                v.author_sig,
                                v.ctr_addr,
                                v.pi,
                                v.sn_1,
                                v.sn_2,
                                v.cm_1,
                                v.cm_2,
                                v.merkle_rt,
                            ));

                        match sys_handle
                            .machine
                            .blockchain
                            .dist_ledger
                            .apis
                            .send_tx(tx_candidate)
                            .await
                        {
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
                                    message: String::from(err.to_string()),
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

            match sys_handle
                .machine
                .blockchain
                .dist_ledger
                .apis
                .get_tx(&body.hash)
                .await
            {
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

                    match sys_handle
                        .machine
                        .blockchain
                        .dist_ledger
                        .apis
                        .get_block(&block_hash)
                    {
                        Ok(_block) => {
                            println!("ok");

                            return SuccessResult {
                                id: String::from("1"),
                                result: String::from(""),
                            }
                            .into_hyper_result();
                        }
                        Err(err) => {
                            println!("err");

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

#[derive(Deserialize, Debug)]
pub(crate) struct CallContractBody {
    ctr_addr: String,
    request: CtrRequest,
}

pub(crate) async fn call_contract(
    req: Request<Body>,
    sys_handle: Arc<SystemHandle>,
) -> Result<Response<Body>, hyper::Error> {
    match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body = match serde_json::from_slice::<CallContractBody>(&b) {
                Ok(b) => b,
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

            match sys_handle
                .machine
                .blockchain
                .dist_ledger
                .apis
                .query_ctr(&body.ctr_addr, body.request)
                .await
            {
                Ok(t) => {
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
