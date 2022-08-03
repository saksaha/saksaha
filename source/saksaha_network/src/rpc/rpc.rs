// use super::server::RPCServer;
use crate::{SaksahaError, SystemHandle};
use sak_rpc_server::{Middleware, RPCServer};
use std::sync::Arc;
use tokio::net::TcpListener;

use super::{router::Router, routes};

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

            Arc::new(router)
        };

        let route = {
            let router_clone = router.clone();

            let m = Middleware::new(Box::new(move |req, res, ctx| {
                router_clone.route(req, res, ctx)
            }));

            m
        };

        let middlewares = vec![route];

        self.server
            .run(self.rpc_socket, self.sys_handle, middlewares)
    }
}
