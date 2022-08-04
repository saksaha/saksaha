use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Response,
};
use log::warn;

pub(crate) fn add_application_json_header(resp: &mut Response<Body>) {
    let headers = resp.headers_mut();

    match HeaderValue::from_str("application/json") {
        Ok(h) => {
            headers.insert(CONTENT_TYPE, h);
        }
        Err(err) => {
            warn!(
                "application/json header has not been initialized, err: {}",
                err
            );
        }
    }
}
