use crate::rpc::routes::v0;
use crate::system::SystemHandle;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

fn get_routes() -> Vec<(Method, &'static str, Handler)> {
    vec![
        (
            Method::POST,
            "/apis/v0/send_transaction",
            Box::new(|req, sys_handle| {
                Box::pin(v0::send_transaction(req, sys_handle))
            }),
        ),
        (
            Method::POST,
            "/apis/v0/get_status",
            Box::new(|req, sys_handle| {
                Box::pin(v0::get_status(req, sys_handle))
            }),
        ),
        (
            Method::POST,
            "/apis/v0/get_transaction",
            Box::new(|req, sys_handle| {
                Box::pin(v0::get_transaction(req, sys_handle))
            }),
        ),
        (
            Method::POST,
            "/apis/v0/get_block",
            Box::new(|req, sys_handle| {
                Box::pin(v0::get_block(req, sys_handle))
            }),
        ),
        (
            Method::POST,
            "/apis/v0/call_contract",
            Box::new(|req, sys_handle| {
                Box::pin(v0::call_contract(req, sys_handle))
            }),
        ),
    ]
}

pub(crate) type Handler = Box<
    dyn Fn(
            Request<Body>,
            Arc<SystemHandle>,
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
        sys_handle: Arc<SystemHandle>,
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

        self.routes[handler_idx].2(req, sys_handle)
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
