use super::{Middleware, StateMachine};
use crate::RPCServerError;
use hyper::{server::conn::AddrIncoming, service, Body, Response, Server};
use log::{debug, error};
use std::{convert::Infallible, sync::Arc};
use tokio::net::TcpListener;

pub struct RPCServer {}

impl RPCServer {
    pub fn run<C>(
        self,
        rpc_socket: TcpListener,
        ctx: C,
        middlewares: Vec<Middleware<C>>,
    ) -> Result<(), RPCServerError>
    where
        C: Clone + Send + Sync + 'static,
    {
        let addr_incoming = match AddrIncoming::from_listener(rpc_socket) {
            Ok(a) => a,
            Err(err) => {
                return Err(format!(
                    "Error initializing Addr Incoming, err: {}",
                    err
                )
                .into());
            }
        };

        let make_svc = service::make_service_fn(move |_conn| {
            // let router = {
            //     let routes = routes::get_routes();
            //     let router = Router::new(routes);

            //     Arc::new(router)
            // };

            // let sys_handle = sys_handle.clone();
            let ctx = ctx.clone();

            // let cors = Middleware::new(Box::new(cors));

            // let route = {
            //     let router_clone = router.clone();

            //     let m = Middleware::new(Box::new(move |req, res, ctx| {
            //         router_clone.route(req, res, ctx)
            //     }));

            //     m
            // };

            let state_machine = {
                let m = StateMachine {
                    // middlewares: vec![cors, route],
                    middlewares: vec![],
                };

                Arc::new(m)
            };

            async move {
                Ok::<_, Infallible>(service::service_fn(move |req| {
                    debug!(
                        "rpc, method: {}, uri: {}",
                        req.method(),
                        req.uri().path()
                    );

                    // let sys_handle_clone = sys_handle.clone();
                    let ctx_clone = ctx.clone();
                    let state_machine_clone = state_machine.clone();
                    let resp: Response<Body> = Response::default();

                    async move {
                        let res =
                            state_machine_clone.run(req, resp, ctx_clone).await;

                        res
                    }
                }))
            }
        });

        tokio::spawn(async move {
            let server = Server::builder(addr_incoming).serve(make_svc);

            if let Err(err) = server.await {
                error!("Error running rpc server, err: {}", err);
            }
        });

        Ok(())
    }
}
