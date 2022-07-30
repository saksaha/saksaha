use super::server::RPCServer;
use crate::{SaksahaError, SystemHandle};
use std::sync::Arc;
use tokio::net::TcpListener;

pub(crate) struct RPCArgs {
    pub sys_handle: Arc<SystemHandle>,
    pub rpc_socket: TcpListener,
}

pub(crate) struct RPC {
    sys_handle: Arc<SystemHandle>,
    rpc_socket: TcpListener,
    server: RPCServer,
}

impl RPC {
    pub(crate) fn init(rpc_args: RPCArgs) -> Result<RPC, String> {
        let server = RPCServer {};

        let rpc = RPC {
            sys_handle: rpc_args.sys_handle,
            rpc_socket: rpc_args.rpc_socket,
            server,
        };

        Ok(rpc)
    }

    pub(crate) async fn run(self) -> Result<(), SaksahaError> {
        self.server.run(self.rpc_socket, self.sys_handle)
    }
}
