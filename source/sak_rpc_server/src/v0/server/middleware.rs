use super::state_machine::HandleResult;
use hyper::{Body, Request, Response};

pub struct Middleware<C>(
    pub  Box<
        dyn Fn(Request<Body>, Response<Body>, C) -> HandleResult<C>
            + Send
            + Sync,
    >,
);

impl<C> Middleware<C> {
    pub fn new(
        f: Box<
            dyn Fn(Request<Body>, Response<Body>, C) -> HandleResult<C>
                + Send
                + Sync,
        >,
    ) -> Middleware<C> {
        Middleware(f)
    }
}
