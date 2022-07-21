use super::{routes::v0, RPCError};
use crate::SystemHandle;
use futures::Future;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::{collections::HashMap, pin::Pin, sync::Arc};

pub(crate) type Handler2 = Box<
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

fn initialize_routes() -> HashMap<&'static str, Handler2> {
    let paths: Vec<Path> = vec![
        Path {
            url: "/apis/v0/send_mint_tx",
            handler: Box::new(|req, sys_handle| {
                Box::pin(v0::send_mint_tx(req, sys_handle))
            }),
        },
        Path {
            url: "/apis/v0/send_pour_tx",
            handler: Box::new(|req, sys_handle| {
                Box::pin(v0::send_pour_tx(req, sys_handle))
            }),
        },
        Path {
            url: "/apis/v0/get_status",
            handler: Box::new(|req, sys_handle| {
                Box::pin(v0::get_status(req, sys_handle))
            }),
        },
        Path {
            url: "/apis/v0/get_transaction",
            handler: Box::new(|req, sys_handle| {
                Box::pin(v0::get_transaction(req, sys_handle))
            }),
        },
        Path {
            url: "/apis/v0/get_block",
            handler: Box::new(|req, sys_handle| {
                Box::pin(v0::get_block(req, sys_handle))
            }),
        },
        Path {
            url: "/apis/v0/call_contract",
            handler: Box::new(|req, sys_handle| {
                Box::pin(v0::call_contract(req, sys_handle))
            }),
        },
    ];

    let mut map = HashMap::new();
    for p in paths.into_iter() {
        map.insert(p.url, p.handler);
    }

    map
}

pub(crate) struct Router {
    route_map: Arc<HashMap<&'static str, Handler2>>,
}

struct Path {
    url: &'static str,
    handler: Handler2,
}

impl Router {
    pub fn new() -> Router {
        let route_map = Arc::new(initialize_routes());

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
