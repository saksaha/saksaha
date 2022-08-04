use crate::MiddlewareResult;
use hyper::{
    header::{self, HeaderValue},
    Body, Method, Request, Response,
};

pub fn cors<C>(
    req: Request<Body>,
    mut resp: Response<Body>,
    ctx: C,
) -> MiddlewareResult<C> {
    let headers = resp.headers_mut();

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

    if req.method() == Method::OPTIONS {
        return MiddlewareResult::End(Box::pin(async { Ok(resp) }));
    }

    MiddlewareResult::Passing(req, resp, ctx)
}
