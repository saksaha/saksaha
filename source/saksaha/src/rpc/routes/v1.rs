use crate::blockchain::ledger::TxValue;
use hyper::{Body, Request, Response, StatusCode};
use serde_json::from_str;

pub(crate) async fn send_transaction(
    req: Request<Body>,
) -> Result<Response<Body>, hyper::Error> {
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
    let body_bytes_vec = body_bytes.to_vec();
    let body_str = std::str::from_utf8(&body_bytes_vec).unwrap();
    let tx_value: TxValue = from_str(body_str).unwrap();

    return Ok(make_not_found_response());
}

pub fn make_not_found_response() -> Response<Body> {
    let mut not_found = Response::default();
    *not_found.status_mut() = StatusCode::NOT_FOUND;

    not_found
}
