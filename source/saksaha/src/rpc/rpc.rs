use super::server::RPCServer;
use crate::machine::Machine;
use crate::p2p::P2PMonitor;
use crate::system::SystemHandle;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub(crate) struct RPC {
    rpc_server: Arc<RPCServer>,
}

pub(crate) struct RPCArgs {
    pub(crate) sys_handle: Arc<SystemHandle>,
    pub(crate) rpc_socket: TcpListener,
    // pub(crate) machine: Arc<Machine>,
    // pub(crate) p2p_monitor: Arc<P2PMonitor>,
}

impl RPC {
    pub(crate) fn init(rpc_args: RPCArgs) -> Result<RPC, String> {
        let rpc_server = {
            let s = RPCServer::init(rpc_args.sys_handle, rpc_args.rpc_socket)?;

            Arc::new(s)
        };

        let rpc = RPC { rpc_server };

        Ok(rpc)
    }

    pub(crate) async fn run(
        &self,
        // rpc_socket: TcpListener,
        // socket_addr: SocketAddr,
    ) {
        let rpc_server = self.rpc_server.clone();

        // let _ = tokio::join!(rpc_server.run(rpc_socket, socket_addr));
        let _ = tokio::join!(rpc_server.run());
    }
}
