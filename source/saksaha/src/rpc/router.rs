use super::{route_map::Handler, routes};
use crate::SystemHandle;
use futures::Future;
use hyper::{Body, Method, Request, Response, StatusCode};
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
                let res = handler(req, sys_handle).await;

                return Ok(make_not_found_response());
            } else {
                println!("not found handler");

                return Ok(make_not_found_response());
            }
        })
    }
}

fn make_not_found_response() -> Response<Body> {
    let mut res: Response<Body> = Response::default();
    *res.status_mut() = StatusCode::NOT_FOUND;
    *res.body_mut() = Body::from("not found");
    res
}
