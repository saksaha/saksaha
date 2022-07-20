use super::router::Router;
use crate::system::{SaksahaError, SystemHandle};
use hyper::server::conn::AddrIncoming;
use hyper::service::{self, Service};
use hyper::{Body, Request, Response, Server, StatusCode};
use log::warn;
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::net::TcpListener;
use tokio::sync::RwLock;

pub(crate) struct RPC {
    pub(crate) sys_handle: Arc<SystemHandle>,
    pub(crate) rpc_socket: TcpListener,
}

pub(crate) struct RPCArgs {
    pub(crate) sys_handle: SystemHandle,
    pub(crate) rpc_socket: TcpListener,
}

impl RPC {
    pub(crate) fn init(rpc_args: RPCArgs) -> Result<RPC, String> {
        let sys_handle = Arc::new(rpc_args.sys_handle);

        let rpc = RPC {
            rpc_socket: rpc_args.rpc_socket,
            sys_handle,
        };

        Ok(rpc)
    }

    pub(crate) async fn run(self) -> Result<(), SaksahaError> {
        let addr_incoming = match AddrIncoming::from_listener(self.rpc_socket) {
            Ok(a) => a,
            Err(err) => {
                return Err(format!(
                    "Error initializing Addr Incoming, err: {}",
                    err
                )
                .into());
            }
        };

        let sys_handle = self.sys_handle.clone();

        let make_svc = service::make_service_fn(move |_conn| {
            let router = Router::new();
            let sys_handle = sys_handle.clone();

            async move {
                let service_fn =
                    Ok::<_, Infallible>(service::service_fn(move |req| {
                        let routed = router.route(req, sys_handle.clone());

                        async move {
                            let res = routed.await;

                            Ok::<Response<Body>, Infallible>(Response::new(
                                "Hello, World!".into(),
                            ))
                        }
                    }));
                service_fn
            }
        });

        tokio::spawn(async move {
            let _hyper_server =
                Server::builder(addr_incoming).serve(make_svc).await;
        });

        Ok(())
    }
}
