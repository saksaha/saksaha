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
    // rpc_server: RPCServer,
    pub(crate) sys_handle: SystemHandle,
    pub(crate) rpc_socket: TcpListener,
}

pub(crate) struct RPCArgs {
    pub(crate) sys_handle: SystemHandle,
    pub(crate) rpc_socket: TcpListener,
}

impl RPC {
    pub(crate) fn init(rpc_args: RPCArgs) -> Result<RPC, String> {
        let rpc = RPC {
            rpc_socket: rpc_args.rpc_socket,
            sys_handle: rpc_args.sys_handle,
        };

        Ok(rpc)
    }

    pub(crate) async fn run(self) -> Result<(), SaksahaError> {
        async fn handle(
            _: Request<Body>,
        ) -> Result<Response<Body>, Infallible> {
            Ok(Response::new("Hello, World!".into()))
        }

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

        let router = Router::new();

        let make_svc = service::make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service::service_fn(handle))
        });

        // let make_svc = MakeSvc {
        //     router,
        //     sys_handle: self.sys_handle,
        // };

        let hyper_server = Server::builder(addr_incoming).serve(make_svc);
        // let mut hyper_server_guard = self.hyper_server.write().await;

        Ok(())
    }
}
