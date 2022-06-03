use super::router::Router;
use crate::system::SystemHandle;
use hyper::server::conn::AddrIncoming;
use hyper::service::Service;
use hyper::{Body, Request, Response, Server};
use log::warn;
use logger::{tinfo, twarn};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::net::TcpListener;
use tokio::sync::RwLock;

pub(crate) struct RPCServer {
    hyper_server: Arc<RwLock<Server<AddrIncoming, MakeSvc>>>,
}

impl RPCServer {
    pub fn init(
        sys_handle: Arc<SystemHandle>,
        rpc_socket: TcpListener,
    ) -> Result<RPCServer, String> {
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
            sys_handle: sys_handle.clone(),
        };

        let hyper_server = {
            let s = Server::builder(addr_incoming).serve(make_svc);

            Arc::new(RwLock::new(s))
        };

        let rpc_server = RPCServer { hyper_server };

        Ok(rpc_server)
    }

    pub async fn run(&self) -> Result<(), String> {
        let mut hyper_server_guard = self.hyper_server.write().await;

        match (&mut *hyper_server_guard).await {
            Ok(_) => {
                warn!("RPC server has stopped");
            }
            Err(err) => {
                return Err(format!("Error while running RPC, err: {}", err));
            }
        }

        Ok(())
    }
}

struct Svc {
    router: Arc<Router>,
    sys_handle: Arc<SystemHandle>,
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
        self.router.route(req, self.sys_handle.clone())
    }
}

struct MakeSvc {
    router: Arc<Router>,
    sys_handle: Arc<SystemHandle>,
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
        let sys_handle = self.sys_handle.clone();

        Box::pin(async { Ok(Svc { router, sys_handle }) })
    }
}
