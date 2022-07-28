mod cors;

pub(in crate::rpc) use cors::*;
use hyper::{Body, Request, Response};

pub(in crate::rpc) type NextFn =
    dyn Fn(Request<Body>, Response<Body>, NFn) -> Response<Body>;

type NFn = dyn Fn(Request<Body>, Response<Body>, NextFn) -> Response<Body>;
