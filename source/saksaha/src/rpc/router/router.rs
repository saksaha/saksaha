use super::utils;
use crate::{
    rpc::{route_map::Handler, routes, RPCError},
    SystemHandle,
};
use futures::Future;
use hyper::{Body, Method, Request, Response};
use std::{collections::HashMap, pin::Pin, sync::Arc};

pub(crate) struct Router {
    route_map: Arc<HashMap<&'static str, Handler>>,
}

impl Router {
    pub fn new() -> Router {
        let route_map = Arc::new(routes::get_routes());

        Router { route_map }
    }

    pub(crate) fn route(
        &self,
        req: Request<Body>,
        sys_handle: Arc<SystemHandle>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Response<Body>, hyper::Error>>
                + Send
                + Sync,
        >,
    > {
        println!("method: {}, req: {}", req.method(), req.uri().path());

        let route_map = self.route_map.clone();

        Box::pin(async move {
            let route_map = route_map.clone();

            if let Some(handler) = route_map.get(req.uri().path()) {
                println!("found handler");

                match handler(req, sys_handle).await {
                    Ok(r) => return Ok(r),
                    Err(err) => {
                        return Ok(utils::make_error_response(err));
                    }
                }
            } else {
                println!("not found handler");

                return Ok(utils::make_not_found_response());
            }
        })
    }
}
