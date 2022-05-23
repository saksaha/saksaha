use crate::machine::Machine;

use super::server::RPCServer;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub(crate) struct RPC {
    rpc_server: Arc<RPCServer>,
}

pub(crate) struct RPCArgs {
    pub(crate) machine: Arc<Machine>,
}

impl RPC {
    pub(crate) fn init(rpc_args: RPCArgs) -> Result<RPC, String> {
        let rpc_server = {
            let s = RPCServer::init(rpc_args.machine)?;

            Arc::new(s)
        };

        let rpc = RPC { rpc_server };

        Ok(rpc)
    }

    pub(crate) async fn run(
        &self,
        rpc_socket: TcpListener,
        socket_addr: SocketAddr,
    ) {
        let rpc_server = self.rpc_server.clone();

        let _ = tokio::join!(rpc_server.run(rpc_socket, socket_addr));
    }
}
