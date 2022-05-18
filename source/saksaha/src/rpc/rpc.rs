use super::server::RPCServer;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct RPC {
    rpc_server: Arc<RPCServer>,
}

impl RPC {
    pub fn init() -> Result<RPC, String> {
        let rpc_server = {
            let s = RPCServer::init()?;

            Arc::new(s)
        };

        let rpc = RPC { rpc_server };

        Ok(rpc)
    }

    pub async fn run(&self, rpc_socket: TcpListener, socket_addr: SocketAddr) {
        let rpc_server = self.rpc_server.clone();

        let _ = tokio::join!(rpc_server.run(rpc_socket, socket_addr));
    }
}
