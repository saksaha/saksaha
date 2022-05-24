use crate::rpc::status_code::Status;
use crate::{blockchain::blockchain::TxValue, machine::Machine};
use hyper::{Body, Request, Response, StatusCode};
use std::convert::TryInto;
use std::{str::Utf8Error, sync::Arc};
pub(crate) async fn send_transaction(
    req: Request<Body>,
    machine: Arc<Machine>,
) -> Result<Response<Body>, hyper::Error> {
    let _ = machine.send_transaction().await;

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

    let s = Status::success_response();
    println!("{:?}", s);
    let j = serde_json::to_string(&s).unwrap();

    *not_found.body_mut() = Body::from(j);

    not_found
}
