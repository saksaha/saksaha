use hyper::server::conn::{AddrIncoming, AddrStream};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use logger::{tdebug, tinfo, twarn};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;
use tokio::task::JoinHandle;

use crate::rpc::apis;

pub(crate) struct RPCServer {
    // pub(crate) rpc_socket: Option<TcpListener>,
// pub(crate) socket_addr: SocketAddr,
}

impl RPCServer {
    pub fn init() -> Result<RPCServer, String> {
        let rpc_server = RPCServer {};

        Ok(rpc_server)
    }

    pub async fn run(
        &self,
        rpc_socket: TcpListener,
        _socket_addr: SocketAddr,
    ) -> Result<(), String> {
        let addr_incoming = match AddrIncoming::from_listener(rpc_socket) {
            Ok(a) => a,
            Err(err) => {
                return Err(format!(
                    "Error initializing Addr Incoming, err: {}",
                    err
                ));
            }
        };

        let make_svc = make_service_fn(|socket: &AddrStream| {
            let remote_addr = socket.remote_addr();

            tdebug!(
                "saksaha",
                "rpc",
                "Incoming request, from: {}",
                remote_addr
            );

            async {
                Ok::<_, Infallible>(service_fn(move |req| {
                    apis::handle_request(req)
                }))
            }
        });

        let server = Server::builder(addr_incoming).serve(make_svc);

        tinfo!("saksaha", "rpc", "Starting rpc server");

        match server.await {
            Ok(_) => {
                twarn!("saksaha", "rpc", "RPC server has stopped");
            }
            Err(err) => {
                return Err(format!("Error while running RPC, err: {}", err));
            }
        };

        Ok(())
    }
}
