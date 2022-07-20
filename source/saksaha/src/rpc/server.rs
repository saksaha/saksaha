use super::router::Router;
use crate::system::SystemHandle;
use hyper::server::conn::AddrIncoming;
use hyper::service::Service;
use hyper::{Body, Request, Response, Server, StatusCode};
use log::warn;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tokio::net::TcpListener;
use tokio::sync::RwLock;

// pub(crate) struct RPCServer {
//     rpc_socket: TcpListener,
//     sys_handle: SystemHandle,
// }

// impl RPCServer {
//     pub fn init(
//         sys_handle: SystemHandle,
//         rpc_socket: TcpListener,
//     ) -> Result<RPCServer, String> {
//         let rpc_server = RPCServer {
//             rpc_socket,
//             sys_handle,
//         };

//         Ok(rpc_server)
//     }

//     pub async fn run(self) -> Result<(), String> {
//         let addr_incoming = match AddrIncoming::from_listener(self.rpc_socket) {
//             Ok(a) => a,
//             Err(err) => {
//                 return Err(format!(
//                     "Error initializing Addr Incoming, err: {}",
//                     err
//                 ));
//             }
//         };

//         let router = {
//             let r = Router::new();
//             // Arc::new(r)
//             r
//         };

//         let make_svc = MakeSvc {
//             router,
//             sys_handle: self.sys_handle,
//         };

//         let hyper_server = Server::builder(addr_incoming).serve(make_svc);
//         // let mut hyper_server_guard = self.hyper_server.write().await;

//         Ok(())
//     }
// }

// struct Svc {
//     router: Router,
//     sys_handle: SystemHandle,
// }

// impl Service<Request<Body>> for Svc {
//     type Response = Response<Body>;
//     type Error = hyper::Error;
//     type Future = Pin<
//         Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>,
//     >;

//     fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, req: Request<Body>) -> Self::Future {
//         self.router.route(req, self.sys_handle)
//     }
// }

struct MakeSvc {
    router: Router,
    sys_handle: SystemHandle,
}

impl<T> Service<T> for MakeSvc {
    type Response = Response<Body>;
    type Error = hyper::Error;
    type Future = Pin<
        Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _: T) -> Self::Future {
        // let router = self.router.clone();
        // let sys_handle = self.sys_handle.clone();
        // let body: Vec<u8> = "hello, world!\n".as_bytes().to_owned();
        // // Create the HTTP response
        // let resp = Response::builder()
        //     .status(StatusCode::OK)
        //     .body(body)
        //     .expect("Unable to create `http::Response`");

        let mut res = Response::default();
        *res.status_mut() = StatusCode::NOT_FOUND;

        let fut = async { Ok(res) };

        Box::pin(fut)
    }
}
