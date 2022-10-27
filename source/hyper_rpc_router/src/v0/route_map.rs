use super::RouteState;
use futures::Future;
use hyper::{Body, Response};
use std::pin::Pin;

pub type MethodName = &'static str;

pub type Params = Option<Vec<u8>>;

pub type Handler<C> = Box<
    dyn Fn(
            RouteState,
            Params,
            C,
        ) -> Pin<Box<dyn Future<Output = Response<Body>> + Send + Sync + 'static>>
        + Send
        + Sync
        + 'static,
>;

pub struct Path<C> {
    pub method: MethodName,
    pub handler: Handler<C>,
}
