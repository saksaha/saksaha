use crate::rpc::RPCError;
use crate::SystemHandle;
use futures::Future;
use hyper::{Body, Request, Response};
use std::pin::Pin;

pub type MethodName = &'static str;

pub type Params = Option<Vec<u8>>;

pub(crate) type Handler<C> = Box<
    dyn Fn(
            Params,
            C,
        ) -> Pin<
            Box<
                dyn Future<Output = Result<Response<Body>, RPCError>>
                    + Send
                    + Sync,
            >,
        > + Send
        + Sync,
>;

pub(crate) struct Path<C> {
    pub method: MethodName,
    pub handler: Handler<C>,
}
