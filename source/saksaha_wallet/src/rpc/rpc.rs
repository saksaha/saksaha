use crate::{
    rpc::{ctx::RouteCtx, routes},
    wallet::Wallet,
    WalletError,
};
use colored::Colorize;
use hyper_rpc_router::Router;
use hyper_server::{cors, HttpServer, Middleware};
use sak_logger::{error, info, warn};
use std::sync::Arc;
use tokio::net::TcpListener;

pub(crate) struct RPC {
    rpc_port: u16,
    rpc_socket: TcpListener,
    wallet: Arc<Wallet>,
}

impl RPC {
    pub async fn init(rpc_port: Option<u16>, wallet: Arc<Wallet>) -> Result<RPC, WalletError> {
        let rpc_port = rpc_port.unwrap_or_else(|| {
            warn!("rpc_port is not provided, default to {}", 36612);

            36612
        });

        let (rpc_socket, socket_addr) = match sak_utils_net::bind_tcp_socket(Some(rpc_port)).await {
            Ok((socket, socket_addr)) => {
                info!(
                    "Bound tcp socket for RPC, addr: {}",
                    socket_addr.to_string(),
                );

                (socket, socket_addr)
            }
            Err(err) => {
                error!(
                    "Could not bind a tcp socket for RPC, port: {}, err: {}",
                    rpc_port, err,
                );

                return Err(err.into());
            }
        };

        let rpc = RPC {
            rpc_port: socket_addr.port(),
            rpc_socket,
            wallet,
        };

        Ok(rpc)
    }

    pub async fn run(self) -> Result<(), WalletError> {
        info!("RPC server runs");

        let router = {
            let routes = routes::get_routes();
            let router = Router::new(routes);

            router
        };

        let cors = Middleware::new(Box::new(cors));

        let route = {
            let m = Middleware::new(Box::new(move |req, res, ctx| router.route(req, res, ctx)));

            m
        };

        let middlewares = vec![cors, route];

        let rpc_server = HttpServer {};

        let ctx = {
            let c = RouteCtx {
                wallet: self.wallet,
            };

            Arc::new(c)
        };

        rpc_server.run(self.rpc_socket, ctx, middlewares).await?;

        Ok(())
    }

    pub fn get_rpc_port(&self) -> u16 {
        self.rpc_port
    }
}
