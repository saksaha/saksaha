use super::router::Router;
use crate::machine::Machine;
use crate::p2p::P2PMonitor;
use crate::system::SystemHandle;
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

pub(crate) struct RPCServer2 {
    // create_server:
//     Box<dyn FnOnce() -> Pin<Box<dyn Future<Output = Result<(), String>>>>>,
// s1: Box<
//     dyn FnOnce(Vec<usize>) -> Pin<Box<dyn Future<Output = Vec<usize>>>>,
// >,
// hyper_server: Arc<Server<AddrIncoming, MakeSvc>>,
// machine: Arc<Machine>,
// p2p_monitor: Arc<P2PMonitor>
}

pub(crate) struct RPCServer {
    pub s1: Box<
        dyn Fn() -> Pin<
                Box<dyn Future<Output = Result<(), String>> + Send + Sync>,
            > + Send
            + Sync,
    >,
}

// impl Services {
//     fn new(
//         f: Box<
//             dyn FnOnce(Vec<usize>) -> Pin<Box<dyn Future<Output = Vec<usize>>>>,
//         >,
//     ) -> Self {
//         Services { s1: f }
//     }
// }

enum NumberOperation {
    AddOne,
    MinusOne,
}

impl RPCServer {
    pub fn init(
        sys_handle: Arc<SystemHandle>,
        rpc_socket: TcpListener,
        // machine: Arc<Machine>,
        // p2p_monitor: Arc<P2PMonitor>,
    ) -> Result<RPCServer, String> {
        let input = vec![1, 2, 3];
        let op = NumberOperation::AddOne;

        // let s = Services::new(Box::new(move |numbers| {
        //     Box::pin(async move { numbers })
        // }));

        let rpc_server = RPCServer {
            s1: Box::new(move || {
                Box::pin(async move {
                    // let addr_incoming =
                    //     match AddrIncoming::from_listener(rpc_socket) {
                    //         Ok(a) => a,
                    //         Err(err) => {
                    //             return Err(format!(
                    //                 "Error initializing Addr Incoming, err: {}",
                    //                 err
                    //             ));
                    //         }
                    //     };

                    // let router = {
                    //     let r = Router::new();
                    //     Arc::new(r)
                    // };

                    // let make_svc = MakeSvc {
                    //     router,
                    //     sys_handle,
                    //     // machine: self.machine.clone(),
                    //     // p2p_monitor: self.monitor.clone(),
                    //     // node: self.node.clone(),
                    // };

                    // let hyper_server =
                    //     Server::builder(addr_incoming).serve(make_svc);

                    // match hyper_server.await {
                    //     Ok(_) => {
                    //         twarn!("saksaha", "rpc", "RPC server has stopped");
                    //     }
                    //     Err(err) => {
                    //         return Err(format!(
                    //             "Error while running RPC, err: {}",
                    //             err
                    //         ));
                    //     }
                    // };

                    Ok(())
                })
            }),
        };

        // let create_server = Box::new(|| {
        //     Box::pin(async move {
        //         // let addr_incoming = match AddrIncoming::from_listener(rpc_socket) {
        //         //     Ok(a) => a,
        //         //     Err(err) => {
        //         //         return Err(format!(
        //         //             "Error initializing Addr Incoming, err: {}",
        //         //             err
        //         //         ));
        //         //     }
        //         // };

        //         // let router = {
        //         //     let r = Router::new();
        //         //     Arc::new(r)
        //         // };

        //         // let make_svc = MakeSvc {
        //         //     router,
        //         //     sys_handle,
        //         //     // machine: self.machine.clone(),
        //         //     // p2p_monitor: self.monitor.clone(),
        //         //     // node: self.node.clone(),
        //         // };

        //         // let hyper_server = Server::builder(addr_incoming).serve(make_svc);

        //         // match hyper_server.await {
        //         //     Ok(_) => {
        //         //         twarn!("saksaha", "rpc", "RPC server has stopped");
        //         //     }
        //         //     Err(err) => {
        //         //         return Err(format!(
        //         //             "Error while running RPC, err: {}",
        //         //             err
        //         //         ));
        //         //     }
        //         // };

        //         Ok(())
        //     })
        // });

        // let create_server = Box::new(create_server);

        // let rpc_server = RPCServer {
        //     // create_server,
        //     // s1,
        //     // hyper_server,
        //     // machine,
        //     // p2p_monitor,
        // };

        Ok(rpc_server)
    }

    pub async fn run(
        &self,
        // rpc_socket: TcpListener,
        // socket_addr: SocketAddr,
    ) -> Result<(), String> {
        println!("running rpc server!!!!!!!!!!!!!!!");

        (&self.s1)().await;

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
    // node: Arc<Node>,
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
        // let node = self.node.clone();
        let sys_handle = self.sys_handle.clone();

        Box::pin(async {
            Ok(Svc {
                router,
                sys_handle,
                // machine,
                // p2p_monitor,
            })
        })
    }
}
