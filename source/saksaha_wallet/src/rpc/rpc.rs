use crate::WalletError;
use colored::Colorize;
use log::{error, info};
use std::time::Duration;
use tokio::net::TcpListener;

pub(crate) struct RPC {
    rpc_port: u16,
    rpc_socket: TcpListener,
}

impl RPC {
    pub async fn init(rpc_port: Option<u16>) -> Result<RPC, WalletError> {
        let (rpc_socket, socket_addr) =
            match sak_utils_net::bind_tcp_socket(rpc_port).await {
                Ok((socket, socket_addr)) => {
                    info!(
                        "Bound tcp socket for RPC, addr: {}",
                        socket_addr.to_string().yellow(),
                    );

                    (socket, socket_addr)
                }
                Err(err) => {
                    error!("Could not bind a tcp socket for RPC, err: {}", err);

                    return Err(err.into());
                }
            };

        let rpc = RPC {
            rpc_port: socket_addr.port(),
            rpc_socket,
        };

        Ok(rpc)
    }

    pub async fn run(self) {
        println!("rpc starts");

        let router = {
            let routes = routes::get_routes();
            let router = Router::new(routes);

            router
        };

        let cors = Middleware::new(Box::new(cors));

        let route = {
            let m = Middleware::new(Box::new(move |req, res, ctx| {
                router.route(req, res, ctx)
            }));

            m
        };

        let middlewares = vec![cors, route];

        self.server
            .run(self.rpc_socket, self.sys_handle, middlewares)
    }
}
