use super::{Middleware, StateMachine};
use crate::{
    rpc::{middlewares::cors, router::Router, routes},
    SaksahaError, SystemHandle,
};
use hyper::{
    server::conn::AddrIncoming, service, Body, Method, Response, Server,
};
use log::{debug, error, info};
use std::{convert::Infallible, pin::Pin, sync::Arc};
use tokio::net::TcpListener;

pub(in crate::rpc) struct RPCServer {}

impl RPCServer {
    pub(crate) fn run(
        self,
        rpc_socket: TcpListener,
        sys_handle: Arc<SystemHandle>,
    ) -> Result<(), SaksahaError> {
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
            let router = {
                let routes = routes::get_routes();
                let router = Router::new(routes);

                Arc::new(router)
            };

            let sys_handle = sys_handle.clone();

            let cors = Middleware::new(Box::new(cors));

            let route = {
                let router_clone = router.clone();

                let m = Middleware::new(Box::new(move |req, res, ctx| {
                    router_clone.route(req, res, ctx)
                }));

                m
            };

            let state_machine = {
                let m = StateMachine {
                    middlewares: vec![cors, route],
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

                    let sys_handle_clone = sys_handle.clone();
                    let state_machine_clone = state_machine.clone();
                    let resp: Response<Body> = Response::default();

                    async move {
                        let res = state_machine_clone
                            .run(req, resp, sys_handle_clone)
                            .await;

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
