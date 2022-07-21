use crate::SystemHandle;

use super::{router::Handler, routes::v0};
use futures::Future;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::{collections::HashMap, pin::Pin, sync::Arc};

fn initialize_routes() -> HashMap<&'static str, Handler> {
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

pub(crate) struct Router2 {
    routes: HashMap<&'static str, Handler>,
}

struct Path {
    url: &'static str,
    handler: Handler,
}

impl Router2 {
    pub fn new() -> Router2 {
        let routes = initialize_routes();

        Router2 { routes }
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

        match self.routes.get(req.uri().path()) {
            Some(h) => h(req, sys_handle),
            None => {
                return Box::pin(async {
                    println!("Not found");

                    return Ok(make_not_found_response());
                });
            }
        }
    }
}

fn make_not_found_response() -> Response<Body> {
    let mut res = Response::default();
    *res.status_mut() = StatusCode::NOT_FOUND;
    res
}
