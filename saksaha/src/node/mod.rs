pub mod error;
pub mod status;

use self::error::NodeError;
use crate::{
    node::status::Status,
    p2p::host::Host,
    pconfig::PConfig,
    process::Process,
    rpc::{self, RPC},
};
use std::sync::Arc;
use log::{debug, error, info};
use tokio::{self, signal};

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
        let host = Host::new();

        let rpc_started = rpc.start();
        let rpc_port: u16 = match rpc_started.await {
            Ok(port) => port,
            Err(err) => {
                return Err(NodeError::SetupFail(format!(
                    "Error joining rpc start thread, err: {}",
                    err
                )));
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
            Err(err) => return Err(NodeError::SetupFail(err)),
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
    ) -> Result<(), String> {
        debug!("Start node...");

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        match runtime {
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
                                debug!("ctrl+k is pressed.");

                                Process::shutdown();
                            },
                            Err(err) => {
                                error!(
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
                let msg = format!("runtime fail, err: {:?}", err);
                return Err(msg);
            }
        };

        Ok(())
    }

    pub fn persist_state(&self) {
        info!("Storing state of node");
    }
}
