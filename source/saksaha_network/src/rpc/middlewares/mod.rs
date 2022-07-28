mod cors;

pub(in crate::rpc) use cors::*;
use futures::Future;
use hyper::{Body, Request, Response};
use std::{pin::Pin, sync::Arc};

pub(in crate::rpc) struct NextFn(
    pub  dyn Fn(
        Request<Body>,
        Response<Body>,
        Arc<NextFn>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Response<Body>, hyper::Error>>
                + Send
                + Sync,
        >,
    >,
);
