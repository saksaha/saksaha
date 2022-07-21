use crate::rpc::routes::v0;
use crate::system::SystemHandle;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub(in crate::rpc) struct Route {
    url: &'static str,
    handler: Handler,
}

pub(crate) type Handler = Box<
    dyn Fn(
            Request<Body>,
            Arc<SystemHandle>,
        ) -> Pin<
            Box<
                dyn Future<Output = Result<Response<Body>, hyper::Error>>
                    + Send
                    + Sync,
            >,
        > + Send
        + Sync,
>;

// struct A<F, Fut>
// where
//     F: Fn(Request<Body>, Arc<SystemHandle>) -> Fut,
//     Fut: Future<Output = Result<Response<Body>, hyper::Error>>,
// {
//     f: F,
// }

// fn get_routes() -> Vec<(Method, &'static str, Handler)> {
fn get_routes() {
    let aa: Vec<Handler> = vec![
        Box::new(|req, sys_handle| Box::pin(v0::send_mint_tx(req, sys_handle))),
        Box::new(|req, sys_handle| Box::pin(v0::send_mint_tx(req, sys_handle))),
    ];

    let mut a: HashMap<&'static str, Handler> = HashMap::new();
    a.insert("aa", Box::new(|req, sys_handle| Box::pin(v0::send_mint_tx(req, sys_handle))));
    a.insert("aa", Box::new(|req, sys_handle| Box::pin(v0::send_mint_tx(req, sys_handle))));

    //     [
    //     // (
    //     //     "/apis/v0/send_mint_tx",
    //     //     Box::new(|req, sys_handle| {
    //     //         Box::pin(v0::send_mint_tx(req, sys_handle))
    //     //     }) as Handler,
    //     // ),
    //     (
    //         "/apis/v0/send_mint_tx",
    //         // 1,
    //         Box::new(|req, sys_handle| {
    //             Box::pin(v0::send_pour_tx(req, sys_handle))
    //         }),
    //     ),
    //     // (
    //     //     "/apis/v0/send_mint_tx",
    //     //     Box::new(|req, sys_handle| {
    //     //         Box::pin(v0::send_pour_tx(req, sys_handle))
    //     //     }),
    //     // ),
    // ]);

    // let v = vec![
    //     (
    //         Method::POST,
    //         "/apis/v0/send_mint_tx",
    //         Box::new(|req, sys_handle| {
    //             Box::pin(v0::send_mint_tx(req, sys_handle))
    //         }),
    //     ),
    //     (
    //         Method::POST,
    //         "/apis/v0/send_pour_tx",
    //         Box::new(|req, sys_handle| {
    //             Box::pin(v0::send_pour_tx(req, sys_handle))
    //         }),
    //     ),
    //     (
    //         Method::POST,
    //         "/apis/v0/get_status",
    //         Box::new(|req, sys_handle| {
    //             Box::pin(v0::get_status(req, sys_handle))
    //         }),
    //     ),
    //     (
    //         Method::POST,
    //         "/apis/v0/get_transaction",
    //         Box::new(|req, sys_handle| {
    //             Box::pin(v0::get_transaction(req, sys_handle))
    //         }),
    //     ),
    //     (
    //         Method::POST,
    //         "/apis/v0/get_block",
    //         Box::new(|req, sys_handle| {
    //             Box::pin(v0::get_block(req, sys_handle))
    //         }),
    //     ),
    //     (
    //         Method::POST,
    //         "/apis/v0/call_contract",
    //         Box::new(|req, sys_handle| {
    //             Box::pin(v0::call_contract(req, sys_handle))
    //         }),
    //     ),
    // ];
    // v
}

pub(crate) struct Router {
    // pub(crate) routes: >,
}

impl Router {
    pub(crate) fn new() -> Router {
        let routes = {
            let r = get_routes();
            // Arc::new(r)
            r
        };

        Router { 
            //routes 
        }
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
