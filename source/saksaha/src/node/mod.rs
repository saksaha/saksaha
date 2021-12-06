pub mod socket;

use crate::{
    p2p::host::Host,
    pconfig::PConfig,
    process::{Process, Shutdown},
    rpc::{self, RPC}, ledger::Ledger,
};
use log::{debug, error, info};
use peer::{PeerStore, Peer};
use std::sync::Arc;
use tokio::{self, signal};
use tokio::sync::Mutex;

pub struct Node {
    inner: Arc<Inner>,
}

impl Node {
    pub fn new() -> Node {
        let inner = Arc::new(Inner {});

        Process::init(inner.clone());

        Node { inner, }
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
        self.inner.start(
            rpc_port,
            disc_port,
            p2p_port,
            bootstrap_endpoints,
            pconfig,
            default_bootstrap_urls,
        )
    }
}

struct Inner;

impl Inner {
    fn start(
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

        match runtime {
            Ok(r) => r.block_on(async {
                match self
                    .start_in_runtime(
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
                return Err(format!("runtime fail, err: {:?}", err));
            }
        };

        Ok(())
    }

    async fn start_in_runtime(
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

        let p2p_host = Host::init(
            pconfig.p2p,
            sockets.rpc.port,
            sockets.p2p,
            disc_port,
            bootstrap_urls,
            default_bootstrap_urls,
        )
        .await?;

        let host_state = p2p_host.host_state.clone();
        let peer_store = host_state.peer_store.clone();

        let ledger = Ledger::new(peer_store);

        rpc.start().await?;
        ledger.start().await?;
        p2p_host.start().await?;

        Ok(())
    }
}

impl Shutdown for Inner {
    fn shutdown(&self) {
        info!("Storing state of node");
    }
}
