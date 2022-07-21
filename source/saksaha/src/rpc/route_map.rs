use super::RPCError;
use crate::SystemHandle;
use futures::Future;
use hyper::{Body, Request, Response};
use std::{collections::HashMap, pin::Pin, sync::Arc};

pub(crate) type Handler = Box<
    dyn Fn(
            Request<Body>,
            Arc<SystemHandle>,
        ) -> Pin<
            Box<
                dyn Future<Output = Result<Response<Body>, RPCError>>
                    + Send
                    + Sync,
            >,
        > + Send
        + Sync,
>;

pub(crate) struct Path {
    pub url: &'static str,
    pub handler: Handler,
}
