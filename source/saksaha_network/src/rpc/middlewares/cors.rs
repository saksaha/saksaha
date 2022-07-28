use futures::Future;
use hyper::{
    header::{self, HeaderValue},
    Body, Request, Response,
};
use std::pin::Pin;

// pub(in crate::rpc) fn cors() -> Pin<
//     Box<
//         dyn Future<Output = Result<Response<Body>, hyper::Error>> + Send + Sync,
//     >,
// > {
//     Box::pin(async { Ok(make_cors_response()) })
// }

pub(in crate::rpc) fn cors(req: Request<Body>, res: Response<Body>) {}

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
