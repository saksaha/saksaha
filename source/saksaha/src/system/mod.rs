mod process;
pub mod socket;

use crate::{ledger::Ledger, p2p::host::Host, pconfig::PConfig, rpc::RPC};
use logger::terr;
use logger::{tdebug, tinfo};
use process::{Process, Shutdown};
use std::sync::Arc;
use tokio::{self, signal};

pub struct System {
    inner: Arc<Inner>,
}

pub struct SystemArgs {
    pub rpc_port: Option<u16>,
    pub disc_port: Option<u16>,
    pub p2p_port: Option<u16>,
    pub bootstrap_urls: Option<Vec<String>>,
    pub pconfig: PConfig,
}

impl System {
    pub fn new() -> System {
        let inner = Arc::new(Inner {});

        Process::init(inner.clone());

        System { inner }
    }

    pub fn start(
        &self,
        sys_args: SystemArgs,
        // rpc_port: Option<u16>,
        // disc_port: Option<u16>,
        // p2p_port: Option<u16>,
        // bootstrap_endpoints: Option<Vec<String>>,
        // pconfig: PConfig,
    ) -> Result<(), String> {
        self.inner.start(
            // rpc_port,
            // disc_port,
            // p2p_port,
            // bootstrap_endpoints,
            // pconfig,
            sys_args,
        )
    }
}

struct Inner;

impl Inner {
    fn start(
        &self,
        // rpc_port: Option<u16>,
        // disc_port: Option<u16>,
        // p2p_port: Option<u16>,
        // bootstrap_endpoints: Option<Vec<String>>,
        // pconfig: PConfig,
        sys_args: SystemArgs,
    ) -> Result<(), String> {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        match runtime {
            Ok(r) => r.block_on(async {
                match self
                    .start_inside_runtime(
                        sys_args,
                        // rpc_port,
                        // disc_port,
                        // p2p_port,
                        // bootstrap_endpoints,
                        // pconfig,
                    )
                    .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        terr!(
                            "saksaha",
                            "system",
                            "Can't start node, err: {}",
                            err,
                        );

                        Process::shutdown();
                    }
                };

                tokio::select!(
                    c = signal::ctrl_c() => {
                        match c {
                            Ok(_) => {
                                tdebug!(
                                    "sahsaha",
                                    "system",
                                    "ctrl+k is pressed.",
                                );

                                Process::shutdown();
                            },
                            Err(err) => {
                                terr!(
                                    "saksaha",
                                    "system",
                                    "Unexpected error while waiting for \
                                        ctrl+p, err: {}",
                                    err,
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

    async fn start_inside_runtime(
        &self,
        sys_args: SystemArgs,
        // rpc_port: Option<u16>,
        // disc_port: Option<u16>,
        // p2p_port: Option<u16>,
        // bootstrap_urls: Option<Vec<String>>,
        // pconfig: PConfig,
    ) -> Result<(), String> {
        tinfo!("saksaha", "system", "");
        tinfo!("saksaha", "system", "System is starting...");

        let sockets =
            socket::setup_sockets(sys_args.rpc_port, sys_args.p2p_port).await?;

        let rpc = RPC::new(sockets.rpc.listener);

        let p2p_host = Host::init(
            sys_args.pconfig.p2p,
            sockets.rpc.port,
            sockets.p2p,
            sys_args.disc_port,
            sys_args.bootstrap_urls,
        )
        .await?;

        let host_state = p2p_host.host_state.clone();
        let peer_store = host_state.peer_store.clone();

        let ledger = Ledger::new(peer_store);

        // rpc.start().await?;
        // ledger.start().await?;
        p2p_host.start().await?;

        Ok(())
    }
}

impl Shutdown for Inner {
    fn shutdown(&self) {
        tinfo!("saksaha", "system", "Storing state of node");
    }
}
