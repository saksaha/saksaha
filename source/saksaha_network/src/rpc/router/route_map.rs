use super::RouteState;
use futures::Future;
use hyper::{Body, Response};
use std::pin::Pin;

pub type MethodName = &'static str;

pub type Params = Option<Vec<u8>>;

pub(in crate::rpc) type Handler<C> = Box<
    dyn Fn(
            RouteState,
            Params,
            C,
        )
            -> Pin<Box<dyn Future<Output = Response<Body>> + Send + Sync>>
        + Send
        + Sync,
>;

pub(in crate::rpc) struct Path<C> {
    pub method: MethodName,
    pub handler: Handler<C>,
}
