use crate::blockchain::blockchain::TxValue;
use hyper::{Body, Request, Response, StatusCode};
use std::str::Utf8Error;
// =======
// use crate::rpc::apis;
// use crate::rpc::router::Router;
// use hyper::{Body, Method, Request, Response, Server, StatusCode, Uri};
// use logger::{tdebug, tinfo, twarn};
// use std::error::Error;
// use std::future::Future;
// >>>>>>> RPC: send transaction api

pub(crate) async fn send_transaction(
    req: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    let body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body_bytes_vec = b.to_vec();
            let body_str = match std::str::from_utf8(&body_bytes_vec) {
                Ok(b) => {
                    let tx_value: TxValue = match serde_json::from_str(b) {
                        Ok(v) => v,
                        Err(err) => {
                            return Ok(make_not_found_response());
                        }
                    };
                }
                Err(err) => {
                    return Ok(make_not_found_response());
                }
            };
        }
        Err(err) => {
            return Ok(make_not_found_response());
        }
    };

    return Ok(make_not_found_response());
}

pub fn make_not_found_response() -> Response<Body> {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;

    not_found
}
