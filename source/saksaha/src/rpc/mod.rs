// use log::{debug, info};
// use std::sync::Arc;
// use tokio::net::TcpListener;

// pub struct RPC {
//     rpc_socket: Arc<TcpListener>,
// }

// impl RPC {
//     pub fn new(rpc_socket: TcpListener, rpc_port: u16) -> RPC {
//         RPC {
//             rpc_socket: Arc::new(rpc_socket),
//         }
//     }

//     pub async fn start(&self) -> Result<u16, String> {
//         info!("Start rpc...");

//         Ok(10000)
//     }
// }

use warp::{Filter, Server};

pub struct HTTPServer<F> {
    http_server: Server<F>,
}

pub async fn new() {
    let routes = warp::any().map(|| "Hello, World!");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
