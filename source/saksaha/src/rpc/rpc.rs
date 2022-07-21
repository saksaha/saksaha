use super::server::RPCServer;
use crate::SystemHandle;
use std::sync::Arc;
use tokio::net::TcpListener;

pub(crate) struct RPC {
    rpc_server: Arc<RPCServer>,
}

pub(crate) struct RPCArgs {
    pub(crate) sys_handle: Arc<SystemHandle>,
    pub(crate) rpc_socket: TcpListener,
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

    pub(crate) async fn run(&self) {
        let rpc_server = self.rpc_server.clone();

        let _ = tokio::join!(rpc_server.run());
    }
}
