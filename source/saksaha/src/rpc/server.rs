use crate::machine::Machine;

use super::node::Node;
use super::router::Router;
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
    node: Arc<Node>,
}

impl RPCServer {
    pub fn init(node: Arc<Node>) -> Result<RPCServer, String> {
        let rpc_server = RPCServer { node };

        Ok(rpc_server)
    }

    pub async fn run(
        &self,
        rpc_socket: TcpListener,
        socket_addr: SocketAddr,
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

        let router = {
            let r = Router::new();
            Arc::new(r)
        };

        let make_svc = MakeSvc {
            router,
            node: self.node.clone(),
        };

        let hyper_server = Server::builder(addr_incoming).serve(make_svc);

        tinfo!(
            "saksaha",
            "rpc",
            "Starting rpc server, socket_addr: {}",
            socket_addr,
        );

        match hyper_server.await {
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

struct Svc {
    router: Arc<Router>,
    node: Arc<Node>,
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
        self.router.route(req, self.node.clone())
    }
}

struct MakeSvc {
    router: Arc<Router>,
    node: Arc<Node>,
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
        let node = self.node.clone();

        Box::pin(async move { Ok(Svc { router, node }) })
    }
}
