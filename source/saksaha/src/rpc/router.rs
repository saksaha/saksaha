use crate::rpc::apis;
use crate::rpc::routes::v1;
use hyper::{body::HttpBody, server::conn::AddrStream, service::Service};
use hyper::{Body, Method, Request, Response, Server, StatusCode, Uri};
use logger::{tdebug, tinfo, twarn};
use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

fn get_routes() -> Vec<(Method, &'static str, Handler)> {
    vec![
        (
            Method::POST,
            "/apis/v1/send_transaction",
            Box::new(|req| Box::pin(v1::send_transaction(req))),
        ),
        // (
        //     Method::GET,
        //     "/apis/v1/get_transaction",
        //     Box::new(|req| Box::pin(v1::get_transaction(req))),
        // ),
    ]
}

pub type Handler = Box<
    dyn Fn(
            Request<Body>,
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

pub(crate) struct RouterService {}

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
    ) -> Pin<
        Box<dyn Future<Output = Result<Response<Body>, hyper::Error>> + Send>,
    > {
        println!("method: {}, req: {}", req.method(), req.uri().path());

        let res = match self.get_handler_idx(req.method(), req.uri().path()) {
            Some(i) => self.routes[i].2(req),
            None => Box::pin(async {
                println!("Not found");

                Ok(v1::make_not_found_response())
            }),
        };
        res
    }

    pub(crate) fn get_handler_idx(
        &self,
        method: &Method,
        path: &str,
    ) -> Option<usize> {
        let routes = self.routes.clone();

        for (idx, (m, p, h)) in routes.iter().enumerate() {
            if m == method && *p == path {
                return Some(idx);
            }
        }

        None
    }
}
