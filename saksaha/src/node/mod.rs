use crate::{
    p2p::host::Host,
    pconfig::PConfig,
    process::Process,
    rpc::{self, RPC},
};
use std::sync::Arc;
use log::{debug, error, info};
use tokio::{self, signal};

pub struct Node;

impl Node {
    pub fn new() -> Node {
        Node {}
    }

    pub fn init(
        &self,
        rpc_port: Option<u16>,
        disc_port: Option<u16>,
        p2p_port: Option<u16>,
        bootstrap_endpoints: Option<Vec<String>>,
        pconfig: PConfig,
        default_bootstrap_urls: String,
    ) -> Result<(), String> {
        info!("Start node...");

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        let _ = match runtime {
            Ok(r) => r.block_on(async {
                match self.start(
                    rpc_port,
                    disc_port,
                    p2p_port,
                    bootstrap_endpoints,
                    pconfig,
                    default_bootstrap_urls,
                ).await {
                    Ok(_) => (),
                    Err(err) => {
                        error!("Can't start node, err: {}", err);

                        Process::shutdown();
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
            }),
            Err(err) => {
                let msg = format!("runtime fail, err: {:?}", err);
                return Err(msg);
            }
        };

        Ok(())
    }

    async fn start(
        &self,
        rpc_port: Option<u16>,
        disc_port: Option<u16>,
        p2p_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
        pconfig: PConfig,
        default_bootstrap_urls: String,
    ) -> Result<(), String> {
        let p2p_config = pconfig.p2p;

        let rpc = RPC::new(rpc_port);
        let rpc_started = rpc.start();
        let rpc_port: u16 = match rpc_started.await {
            Ok(port) => port,
            Err(err) => {
                return Err(format!(
                    "Error joining rpc start thread, err: {}",
                    err
                ));
            }
        };

        let host = Host::init(
            p2p_config,
            rpc_port,
            p2p_port,
            disc_port,
            bootstrap_urls,
            default_bootstrap_urls,
        ).await?;
        host.start().await?;

        Ok(())
    }

    pub fn persist_state(&self) {
        info!("Storing state of node");
    }
}
