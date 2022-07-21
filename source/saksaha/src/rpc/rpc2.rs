use super::{router::Router, router2::Router2, server::RPCServer, RPCArgs};
use crate::{SaksahaError, SystemHandle};
use hyper::{
    server::conn::AddrIncoming, service, Body, Request, Response, Server,
};
use log::{error, info};
use std::{convert::Infallible, sync::Arc};
use tokio::net::TcpListener;

pub(crate) struct RPC2 {
    sys_handle: Arc<SystemHandle>,
    rpc_socket: TcpListener,
    server: RPCServer2,
}

impl RPC2 {
    pub(crate) fn init(rpc_args: RPCArgs) -> Result<RPC2, String> {
        let server = RPCServer2 {};

        let rpc = RPC2 {
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

struct RPCServer2 {}

impl RPCServer2 {
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

        let sys_handle = sys_handle.clone();
        let make_svc = service::make_service_fn(move |_conn| {
            let router = {
                let r = Router2::new();
                r
            };

            let sys_handle_clone = sys_handle.clone();

            async {
                Ok::<_, Infallible>(service::service_fn(move |req| {
                    let res = router.route(req, sys_handle_clone.clone());

                    async {
                        let a = res.await;
                        a
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

// async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
//     Ok(Response::new("Hello, World!".into()))
// }
