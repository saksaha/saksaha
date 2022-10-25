use super::{Middleware, StateMachine};
use crate::RPCServerError;
use futures::Future;
use hyper::{
    server::conn::AddrIncoming,
    service::{self, make_service_fn, service_fn},
    Body, Method, Request, Response, Server, StatusCode,
};
use sak_logger::{debug, error};
use std::{convert::Infallible, sync::Arc};
use tokio::net::TcpListener;

pub struct TCPServer {}

impl TCPServer {
    pub async fn run<C, F, R, S>(
        self,
        tcp_socket: TcpListener,
        ctx: C,
        middlewares: Vec<Middleware<C>>,
        f: F,
    ) -> Result<(), RPCServerError>
    where
        C: Clone + Send + Sync + 'static,
        F: FnMut(Request<R>) -> S + Send,
        S: Future + Send,
    {
        let addr_incoming = match AddrIncoming::from_listener(tcp_socket) {
            Ok(a) => a,
            Err(err) => {
                return Err(format!("Error initializing Addr Incoming, err: {}", err).into());
            }
        };

        // let middlewares = Arc::new(middlewares);

        // let make_svc = service::make_service_fn(move |_conn| {
        //     let ctx = ctx.clone();

        //     let state_machine = {
        //         let m = StateMachine {
        //             middlewares: middlewares.clone(),
        //         };

        //         Arc::new(m)
        //     };

        //     async move {
        //         Ok::<_, Infallible>(service::service_fn(move |req| {
        //             // debug!(
        //             //     "rpc, method: {}, uri: {}",
        //             //     req.method(),
        //             //     req.uri().path()
        //             // );

        //             let ctx_clone = ctx.clone();
        //             let state_machine_clone = state_machine.clone();
        //             let resp: Response<Body> = Response::default();

        //             async move {
        //                 let res = state_machine_clone.run(req, resp, ctx_clone).await;

        //                 res
        //             }
        //         }))
        //     }
        // });

        // let server = Server::builder(addr_incoming).serve(make_svc);

        // if let Err(err) = server.await {
        //     error!("Error running rpc server, err: {}", err);
        // }

        let server = Server::builder(addr_incoming);

        server.serve(make_service_fn(|_| async {
            Ok::<_, hyper::Error>(service_fn(
                f, // |req| async move {
                  // match (req.method(), req.uri().path()) {
                  //     (&Method::POST, "/echo") => {
                  //         let mut response = Response::new(Body::empty());
                  //         *response.body_mut() = Body::from("Try POSTing data to /echo");
                  //         Ok(response)
                  //     }
                  //     _ => {
                  //         let mut not_found = Response::new(Body::empty());
                  //         *not_found.status_mut() = StatusCode::NOT_FOUND;
                  //         Ok::<_, hyper::Error>(not_found)
                  //     }
                  // }
            ))
        }));

        Ok(())
    }
}

async fn a(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let mut not_found = Response::new(Body::empty());
    *not_found.status_mut() = StatusCode::NOT_FOUND;
    Ok::<_, hyper::Error>(not_found)
}
