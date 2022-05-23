""use crate::blockchain::blockchain::TxValue;
use hyper::{Body, Request, Response, StatusCode};
use serde_json::from_str;
use std::str::{from_utf8, Utf8Error};

pub(crate) async fn send_transaction(
    req: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    let body = match hyper::body::to_bytes(req.into_body()).await {
        Ok(b) => {
            let body_bytes_vec = b.to_vec();
            let body_str = match from_utf8(&body_bytes_vec) {
                Ok(b) => {
                    let tx_value: TxValue = from_str(b).unwrap();
                }
                Err(err) => return Err(err)
            };
        }
        Err(err) => {
            return Err(err);
        }
    };

    return Ok(make_not_found_response());
}

pub fn make_not_found_response() -> Response<Body> {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;

    not_found
}