pub mod status;
pub mod error;

use crate::{
    node::status::Status,
    p2p::host::{self, Host, HostError},
    pconfig::PConfig,
    process::Process,
    rpc::{self, RPC},
};
use logger::log;
use std::sync::Arc;
use tokio::{self, signal};
use self::error::NodeError;

pub struct Node {}

impl Node {
    pub fn new() -> Node {
        Node {}
    }

    async fn start_components(
        &self,
        rpc_port: Option<u16>,
        disc_port: Option<u16>,
        bootstrap_endpoints: Option<Vec<String>>,
        pconfig: PConfig,
        default_bootstrap_urls: &str,
    ) -> Result<(), NodeError> {
        let rpc = RPC::new(rpc_port);

        let p2p_config = pconfig.p2p;
        let host = match Host::new() {
            Ok(h) => h,
            Err(err) => return Err(NodeError::InitError(err)),
        };

        let rpc_status = tokio::spawn(async move {
            return rpc.start().await;
        });

        let rpc_port: u16 = match rpc_status.await {
            Ok(status) => match status {
                rpc::Status::Launched(port) => port,
                rpc::Status::SetupFailed(err) => return Err(err),
            },
            Err(err) => {
                return Err(Error::Default(format!("Error joining rpc start thread, err: {}", err)));
            }
        };

        let host_started = host.start(
            p2p_config,
            rpc_port,
            disc_port,
            bootstrap_endpoints,
            default_bootstrap_urls,
        );

        match host_started.await {
            Ok(_) => (),
            HostError::SetupFail(err) => return Err(err),
        };

        Ok(())
    }

    pub fn start(
        self: Arc<Self>,
        rpc_port: Option<u16>,
        disc_port: Option<u16>,
        bootstrap_endpoints: Option<Vec<String>>,
        pconfig: PConfig,
        default_bootstrap_urls: &str,
    ) -> Status<Error> {
        log!(DEBUG, "Start node...\n");

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        let node_status = match runtime {
            Ok(r) => r.block_on(async {
                let started = self.start_components(
                    rpc_port,
                    disc_port,
                    bootstrap_endpoints,
                    pconfig,
                    default_bootstrap_urls,
                );

                match started.await {
                    Ok(_) => (),
                    Err(err) => {
                        return Status::SetupFailed(err);
                    }
                };

                tokio::select!(
                    c = signal::ctrl_c() => {
                        match c {
                            Ok(_) => {
                                log!(DEBUG, "ctrl+k is pressed.\n");

                                Process::shutdown();
                            },
                            Err(err) => {
                                log!(
                                    DEBUG,
                                    "Unexpected error while waiting for \
                                        ctrl+p, err: {}",
                                    err
                                );

                                Process::shutdown();
                            }
                        }
                    },
                );

                return Status::Launched;
            }),
            Err(err) => {
                return Status::SetupFailed(err.into());
            }
        };

        node_status
    }

    pub fn persist_state(&self) {
        log!(DEBUG, "Storing state of node\n");
    }
}
