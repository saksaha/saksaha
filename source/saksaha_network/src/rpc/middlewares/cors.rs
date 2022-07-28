use super::NextFn;
use futures::Future;
use hyper::{
    header::{self, HeaderValue},
    Body, Method, Request, Response,
};
use std::{pin::Pin, sync::Arc};

pub(in crate::rpc) fn cors(
    req: Request<Body>,
    res: Response<Body>,
    next: Arc<NextFn>,
) -> Pin<
    Box<
        dyn Future<Output = Result<Response<Body>, hyper::Error>> + Send + Sync,
    >,
> {
    // if req.method() == Method::OPTIONS {
    //     return Ok();
    // }

    // let f = &next.0;
    // f(req, res, next.clone)
    let n = next.clone();
    return (n.0)(req, res, next); // route
}

fn make_cors_response() -> Response<Body> {
    let mut res = Response::default();

    {
        let headers = res.headers_mut();

        headers.insert(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("*"),
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_HEADERS,
            HeaderValue::from_static("*"),
        );
        headers.insert(
            header::ACCESS_CONTROL_EXPOSE_HEADERS,
            HeaderValue::from_static("*"),
        );
    }

    res
}
