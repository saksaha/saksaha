use log::{debug, info};
use std::sync::Arc;
use tokio::net::TcpListener;
use warp::{Filter, Server};

pub struct RPC {
    rpc_socket: Arc<TcpListener>,
}

// pub struct HTTPServer<F> {
//     http_server: Server<F>,
// }

impl RPC {
    pub async fn init(rpc_socket: TcpListener, rpc_port: u16) -> RPC {
        println!("rpi init() 123123");

        let routes = warp::any().map(|| "Hello, World!");

        // sendTransaction()
        // ..

        let a = warp::serve(routes);
        // a.builder()

        // .run(([127, 0, 0, 1], 3030)).await;

        // warp::hyper::Server::builder()

        RPC {
            rpc_socket: Arc::new(rpc_socket),
        }
    }

    pub async fn start(&self) -> Result<u16, String> {
        info!("Start rpc...");

        Ok(10000)
    }
}
