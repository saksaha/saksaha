pub mod socket;

use crate::{p2p::Host, pconfig::PConfig, process::Process, rpc::{self, RPC}};
use log::{debug, error, info};
use std::sync::Arc;
use tokio::{self, signal};

pub struct Node;

impl Node {
    pub fn new() -> Node {
        Node {}
    }

    pub fn start(
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
                match self
                    .init_and_start(
                        rpc_port,
                        disc_port,
                        p2p_port,
                        bootstrap_endpoints,
                        pconfig,
                        default_bootstrap_urls,
                    )
                    .await
                {
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

    async fn init_and_start(
        &self,
        rpc_port: Option<u16>,
        disc_port: Option<u16>,
        p2p_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
        pconfig: PConfig,
        default_bootstrap_urls: String,
    ) -> Result<(), String> {
        let sockets = socket::setup_sockets(rpc_port, p2p_port).await?;

        let rpc = RPC::new(sockets.rpc.listener);

        let host = Host::init(
            pconfig.p2p,
            sockets.rpc.port,
            sockets.p2p,
            disc_port,
            bootstrap_urls,
            default_bootstrap_urls,
        )
        .await?;

        rpc.start().await?;
        host.start().await?;

        Ok(())
    }

    pub fn persist_state(&self) {
        info!("Storing state of node");
    }
}
