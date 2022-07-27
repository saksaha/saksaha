use super::{router::Router, routes};
use crate::{SaksahaError, SystemHandle};
use hyper::{server::conn::AddrIncoming, service, Server};
use log::{error, info};
use std::{convert::Infallible, sync::Arc};
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

        let sys_handle = sys_handle.clone();
        let make_svc = service::make_service_fn(move |_conn| {
            let routes = Arc::new(routes::get_routes());

            let router = {
                let r: Router<Arc<SystemHandle>> = Router::new(routes);
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