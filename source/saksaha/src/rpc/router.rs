use crate::machine::Machine;
use crate::rpc::routes::v1;
use hyper::{body::HttpBody, server::conn::AddrStream, service::Service};
use hyper::{Body, Method, Request, Response, Server, StatusCode, Uri};
use logger::{tdebug, tinfo, twarn};
use p2p_discovery::Discovery;
use serde::Serialize;
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

fn get_routes() -> Vec<(Method, &'static str, Handler)> {
    vec![
        (
            Method::POST,
            "/apis/v1/send_transaction",
            Box::new(|req, machine| {
                Box::pin(v1::send_transaction(req, machine))
            }),
        ),
        (
            Method::POST,
            "/apis/v1/dummy",
            Box::new(|req, machine| Box::pin(v1::dummy(req, machine))),
        ),
        (
            Method::POST,
            "/apis/v1/get_status",
            Box::new(|req, machine| {
                Box::pin(v1::get_status(req, machine)) //
            }),
        ),
    ]
}

pub(crate) type Handler = Box<
    dyn Fn(
            Request<Body>,
            Arc<Machine>,
        ) -> Pin<
            Box<
                dyn Future<Output = Result<Response<Body>, hyper::Error>>
                    + Send,
            >,
        > + Send
        + Sync,
>;

pub(crate) struct Router {
    pub(crate) routes: Arc<Vec<(Method, &'static str, Handler)>>,
}

impl Router {
    pub(crate) fn new() -> Router {
        let routes = {
            let r = get_routes();
            Arc::new(r)
        };

        Router { routes }
    }

    pub(crate) fn route(
        &self,
        req: Request<Body>,
        machine: Arc<Machine>,
    ) -> Pin<
        Box<dyn Future<Output = Result<Response<Body>, hyper::Error>> + Send>,
    > {
        println!("method: {}, req: {}", req.method(), req.uri().path());

        let handler_idx =
            match self.get_handler_idx(req.method(), req.uri().path()) {
                Some(i) => i,
                None => {
                    return Box::pin(async {
                        println!("Not found");

                        return Ok(make_not_found_response());
                    });
                }
            };

        // Box::pin(async { self.routes[handler_idx].2(req, machine) })
        self.routes[handler_idx].2(req, machine)
    }

    pub(crate) fn get_handler_idx(
        &self,
        method: &Method,
        path: &str,
    ) -> Option<usize> {
        let routes = self.routes.clone();

        for (idx, (m, p, _)) in routes.iter().enumerate() {
            if m == method && *p == path {
                return Some(idx);
            }
        }

        None
    }
}

fn make_not_found_response() -> Response<Body> {
    let mut res = Response::default();
    *res.status_mut() = StatusCode::NOT_FOUND;
    res
}
