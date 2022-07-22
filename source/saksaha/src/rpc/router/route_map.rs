use crate::SystemHandle;
use crate::rpc::RPCError;
use futures::Future;
use hyper::{Body, Request, Response};
use std::{collections::HashMap, pin::Pin, sync::Arc};

pub(crate) type Handler<C> = Box<
    dyn Fn(
            Request<Body>,
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
    pub method: &'static str,
    pub handler: Handler<C>,
}
