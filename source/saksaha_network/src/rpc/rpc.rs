use super::routes;
use crate::{SaksahaError, SystemHandle};
use hyper_rpc_router::Router;
use hyper_server::{cors, Middleware, RPCServer};
use std::sync::Arc;
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
        let router = {
            let routes = routes::get_routes();
            let router = Router::new(routes);

            router
        };

        let cors = Middleware::new(Box::new(cors));

        let route = {
            let m = Middleware::new(Box::new(move |req, res, ctx| {
                router.route(req, res, ctx)
            }));

            m
        };

        let middlewares = vec![cors, route];

        self.server
            .run(self.rpc_socket, self.sys_handle, middlewares)
            .await
    }
}
