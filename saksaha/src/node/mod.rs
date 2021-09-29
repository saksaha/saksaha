pub mod task_manager;

use crate::{
    common::{Error, SakResult},
    err_res,
    p2p::host::Host,
    rpc::RPC,
};
use logger::log;
use std::sync::Arc;
use task_manager::{MsgKind, TaskManager};
use tokio::{self, signal, sync::mpsc, task::JoinHandle};

pub struct Node {
    rpc_port: usize,
    disc_port: usize,
    bootstrap_peers: Option<Vec<String>>,
    public_key: String,
    secret: String,
}

impl Node {
    pub fn new(
        rpc_port: usize,
        disc_port: usize,
        bootstrap_peers: Option<Vec<String>>,
        public_key: String,
        secret: String,
    ) -> SakResult<Node> {
        let node = Node {
            rpc_port,
            disc_port,
            bootstrap_peers,
            public_key,
            secret,
        };

        Ok(node)
    }

    pub fn shutdown(&self) {
        println!("shut down");

        std::process::exit(1);
    }

    pub fn make_host(&self, task_mng: Arc<TaskManager>) -> SakResult<Host> {
        let host = Host::new(
            self.rpc_port,
            self.disc_port,
            self.bootstrap_peers.to_owned(),
            self.public_key.to_owned(),
            self.secret.to_owned(),
            task_mng,
        );
        host
    }

    pub fn make_rpc(&self, task_mng: Arc<TaskManager>) -> SakResult<RPC> {
        let rpc = RPC::new(task_mng);
        Ok(rpc)
    }

    pub fn start(&self) -> SakResult<bool> {
        log!(DEBUG, "Start node...\n");

        let runtime = match tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
        {
            Ok(r) => r.block_on(async {
                let task_mng = Arc::new(TaskManager::new());
                let task_mng_clone = task_mng.clone();

                let host = match self.make_host(task_mng_clone) {
                    Ok(h) => h,
                    Err(err) => {
                        return err_res!("Error making host, err: {}", err);
                    }
                };

                let task_mng_clone = task_mng.clone();

                let rpc = match self.make_rpc(task_mng_clone) {
                    Ok(r) => r,
                    Err(err) => {
                        return err_res!("Error making rpc, err: {}", err);
                    }
                };

                tokio::join!(host.start(), rpc.start(),);

                tokio::select!(
                    msg_kind = task_mng.start_receiving() => {
                        if let MsgKind::SetupFailure = msg_kind {
                            self.shutdown();
                        }
                    },
                    c = signal::ctrl_c() => {
                        match c {
                            Ok(_) => {
                                log!(DEBUG, "ctrl+k is pressed.\n");
                                self.shutdown();
                            }
                            Err(err) => {
                                log!(
                                    DEBUG,
                                    "Unexpected error while waiting for \
                                        ctrl+p, err: {}",
                                    err
                                );

                                self.shutdown();
                            }
                        }
                    },
                );

                Ok(true)
            }),
            Err(err) => {
                return err_res!(
                    "Cannot start the async runtime, err: {}",
                    err
                );
            }
        };

        runtime
    }
}
