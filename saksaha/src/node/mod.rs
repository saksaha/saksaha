pub mod status;
pub mod task_manager;

use crate::{
    common::{Error, Result},
    err,
    node::status::Status,
    p2p::host::{self, Host},
    pconfig::PConfig,
    rpc::{self, RPC},
};
use logger::log;
use std::sync::Arc;
use task_manager::{MsgKind, TaskManager};
use tokio::{self, runtime::Runtime, signal};

pub struct Components {
    rpc: RPC,
    host: Host,
}

pub struct Node {
    rpc_port: Option<u16>,
    disc_port: Option<u16>,
    bootstrap_urls: Option<Vec<String>>,
    pconfig: PConfig,
    task_mng: Arc<TaskManager>,
}

impl Node {
    pub fn new(
        rpc_port: Option<u16>,
        disc_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
        pconfig: PConfig,
    ) -> Result<Node> {
        let task_mng = Arc::new(TaskManager::new());

        let node = Node {
            rpc_port,
            disc_port,
            bootstrap_urls,
            pconfig,
            task_mng,
        };

        Ok(node)
    }

    pub async fn start_components(
        &self,
        components: Arc<Components>,
    ) -> Result<()> {
        let c = components.clone();
        let rpc_status = tokio::spawn(async move {
            return c.rpc.start().await;
        });

        let rpc_port: u16 = match rpc_status.await {
            Ok(status) => match status {
                rpc::Status::Launched(port) => port,
                rpc::Status::SetupFailed(err) => return Err(err),
            },
            Err(err) => {
                return err!("Error joining rpc start thread, err: {}", err);
            }
        };

        let c = components.clone();
        let host_status = tokio::spawn(async move {
            return c.host.start(rpc_port).await;
        });

        match host_status.await {
            Ok(status) => match status {
                host::Status::Launched => {}
                host::Status::SetupFailed(err) => return Err(err),
            },
            Err(err) => {
                return err!("Error joining host start thread, err: {}", err);
            }
        };

        Ok(())
    }

    pub fn make_components(&self) -> Result<Components> {
        let rpc = RPC::new(self.task_mng.clone(), self.rpc_port);

        let secret = self.pconfig.p2p.secret.to_owned();
        let public_key = self.pconfig.p2p.public_key.to_owned();

        let host = Host::new(
            self.disc_port,
            self.bootstrap_urls.to_owned(),
            self.task_mng.clone(),
            secret,
            public_key,
        );

        let components = Components { rpc, host };

        Ok(components)
    }

    pub fn start(&self) -> Status<Error> {
        log!(DEBUG, "Start node...\n");

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        let node_status = match runtime {
            Ok(r) => r.block_on(async {
                let components = match self.make_components() {
                    Ok(c) => Arc::new(c),
                    Err(err) => {
                        return Status::SetupFailed(err);
                    }
                };

                match self.start_components(components).await {
                    Ok(_) => (),
                    Err(err) => {
                        return Status::SetupFailed(err);
                    }
                };

                let task_mng = self.task_mng.clone();

                tokio::select!(
                    msg_kind = task_mng.clone().start_receiving() => {
                        if let MsgKind::SetupFailure = msg_kind {
                            task_mng.shutdown_program();
                        }
                    },
                    c = signal::ctrl_c() => {
                        match c {
                            Ok(_) => {
                                log!(DEBUG, "ctrl+k is pressed.\n");

                                task_mng.shutdown_program();
                            },
                            Err(err) => {
                                log!(
                                    DEBUG,
                                    "Unexpected error while waiting for \
                                        ctrl+p, err: {}",
                                    err
                                );

                                task_mng.shutdown_program();
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
}
