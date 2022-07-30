use crate::rpc::RPCError;
use hyper::{Body, Response, StatusCode};

#[derive(Debug)]
pub(in crate::rpc) struct RouterError {
    err: RPCError,
    res: Response<Body>,
}

impl RouterError {
    pub fn new(err: RPCError, res: Response<Body>) -> RouterError {
        RouterError { err, res }
    }
}

impl std::error::Error for RouterError {}

impl std::fmt::Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RouterError (err: {})", &self.err,)
    }
}
