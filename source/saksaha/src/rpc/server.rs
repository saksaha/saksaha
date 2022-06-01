use crate::machine::Machine;
use crate::p2p::P2PMonitor;

use super::node::Node;
use super::router::Router;
use super::sys_handle::SystemHandle;
use hyper::server::conn::AddrIncoming;
use hyper::service::Service;
use hyper::{Body, Request, Response, Server};
use logger::{tinfo, twarn};
use std::future::Future;
use std::net::SocketAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::net::TcpListener;

pub(crate) struct RPCServer {
    // machine: Arc<Machine>,
// p2p_monitor: Arc<P2PMonitor>,
}

impl RPCServer {
    pub fn init(
        system_handle: SystemHandle,
        rpc_socket: TcpListener,
        // machine: Arc<Machine>,
        // p2p_monitor: Arc<P2PMonitor>,
    ) -> Result<RPCServer, String> {
        let rpc_server = RPCServer {
            // machine,
            // p2p_monitor,
        };

        let addr_incoming = match AddrIncoming::from_listener(rpc_socket) {
            Ok(a) => a,
            Err(err) => {
                return Err(format!(
                    "Error initializing Addr Incoming, err: {}",
                    err
                ));
            }
        };

        let router = {
            let r = Router::new();
            Arc::new(r)
        };

        let make_svc = MakeSvc {
            router,
            // machine: self.machine.clone(),
            // p2p_monitor: self.monitor.clone(),
            // node: self.node.clone(),
        };

        Ok(rpc_server)
    }

    pub async fn run(
        &self,
        rpc_socket: TcpListener,
        socket_addr: SocketAddr,
    ) -> Result<(), String> {
        // let addr_incoming = match AddrIncoming::from_listener(rpc_socket) {
        //     Ok(a) => a,
        //     Err(err) => {
        //         return Err(format!(
        //             "Error initializing Addr Incoming, err: {}",
        //             err
        //         ));
        //     }
        // };

        // let router = {
        //     let r = Router::new();
        //     Arc::new(r)
        // };

        // let make_svc = MakeSvc {
        //     router,
        //     machine: self.machine.clone(),
        //     p2p_monitor: self.monitor.clone(),
        //     // node: self.node.clone(),
        // };

        // let hyper_server = Server::builder(addr_incoming).serve(make_svc);

        // tinfo!(
        //     "saksaha",
        //     "rpc",
        //     "Starting rpc server, socket_addr: {}",
        //     socket_addr,
        // );

        // match hyper_server.await {
        //     Ok(_) => {
        //         twarn!("saksaha", "rpc", "RPC server has stopped");
        //     }
        //     Err(err) => {
        //         return Err(format!("Error while running RPC, err: {}", err));
        //     }
        // };

        Ok(())
    }
}

struct Svc {
    router: Arc<Router>,
    // node: Arc<Node>,
    system_handle: Arc<SystemHandle>,
}

impl Service<Request<Body>> for Svc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        self.router.route(req, self.system_handle.clone())
    }
}

struct MakeSvc {
    router: Arc<Router>,
    // node: Arc<Node>,
    system_handle: SystemHandle,
}

impl<T> Service<T> for MakeSvc {
    type Response = Svc;
    type Error = hyper::Error;
    type Future = Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        let router = self.router.clone();
        // let node = self.node.clone();
        let system_handle = self.system_handle.clone();

        Box::pin(async {
            Ok(Svc {
                router,
                system_handle,
                // machine,
                // p2p_monitor,
            })
        })
    }
}
