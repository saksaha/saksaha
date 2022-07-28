use super::{
    middlewares::make_cors,
    router::{utils, Router},
    routes,
};
use crate::{
    rpc::middlewares::{cors, Middleware, StateMachine},
    SaksahaError, SystemHandle,
};
use futures::Future;
use hyper::{
    server::conn::AddrIncoming, service, Body, Method, Response, Server,
};
use log::{debug, error, info};
use std::{convert::Infallible, pin::Pin, sync::Arc};
use tokio::net::TcpListener;

pub(crate) struct RPCArgs {
    pub sys_handle: Arc<SystemHandle>,
    pub rpc_socket: TcpListener,
}

pub(crate) struct RPC {
    sys_handle: Arc<SystemHandle>,
    rpc_socket: TcpListener,
    server: RPCServer,
}

impl RPC {
    pub(crate) fn init(rpc_args: RPCArgs) -> Result<RPC, String> {
        let server = RPCServer {};

        let rpc = RPC {
            sys_handle: rpc_args.sys_handle,
            rpc_socket: rpc_args.rpc_socket,
            server,
        };

        Ok(rpc)
    }

    pub(crate) async fn run(self) -> Result<(), SaksahaError> {
        self.server.run(self.rpc_socket, self.sys_handle)
    }
}

struct RPCServer {}

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
            println!("111");

            let router = {
                let routes = routes::get_routes();
                let router = Router::new(routes);

                Arc::new(router)
            };

            let sys_handle = sys_handle.clone();

            let c = Middleware(Box::new(cors));

            let r = Middleware(Box::new(|req, res, ctx| {
                router.route(req, res, ctx)
            }));

            let state_machine = {
                let m = StateMachine {
                    middlewares: vec![c],
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
                    let router_clone = router.clone();
                    let state_machine_clone = state_machine.clone();

                    async move {
                        let mut resp: Response<Body> = Response::default();

                        let res = state_machine_clone
                            .run(req, resp, sys_handle_clone)
                            .await;

                        // let res = router_clone
                        //     .clone()
                        //     .route(req, res, sys_handle_clone.clone())
                        //     .await;
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
