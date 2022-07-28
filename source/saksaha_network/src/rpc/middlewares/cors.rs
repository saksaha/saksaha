use super::{HandleResult, Middleware};
use futures::Future;
use hyper::{
    header::{self, HeaderValue},
    Body, Method, Request, Response,
};
use std::{pin::Pin, sync::Arc};

pub(in crate::rpc) fn cors<C>(
    req: Request<Body>,
    res: Response<Body>,
    ctx: C,
) -> HandleResult<C> {
    println!("cors!!!");
    // if req.method() == Method::OPTIONS {
    //     return Ok();
    // }

    HandleResult::Passing(req, res, ctx)
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
