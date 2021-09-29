use std::sync::{Arc,};
use super::{
    discovery::Disc,
    peer_op::PeerOp,
    peer_store::PeerStore,
};
use crate::{common::SakResult, err_res, node::task_manager::{Msg, MsgType, TaskManager}, sync::Sync};
use clap;
use futures::poll;
use logger::log;
use tokio::{sync::{Mutex, mpsc::Sender}, task::JoinHandle};

pub struct Host {
    rpc_port: usize,
    disc_port: usize,
    bootstrap_peers: Option<Vec<String>>,
    task_mng: Arc<TaskManager>,
}

impl Host {
    pub fn new(
        rpc_port: usize,
        disc_port: usize,
        bootstrap_peers: Option<Vec<String>>,
        public_key: String,
        secret: String,
        task_mng: Arc<TaskManager>,
    ) -> SakResult<Host> {
        let host = Host {
            rpc_port,
            disc_port,
            bootstrap_peers,
            task_mng,
        };

        Ok(host)
    }
}

impl Host {
    pub async fn start(&self) {
        log!(DEBUG, "Start host...\n");

        let peer_store = Arc::new(Mutex::new(PeerStore::new(10)));
        let peer_store_clone = peer_store.clone();

        let task_mng = self.task_mng.clone();

        let disc = Disc::new(
            self.disc_port,
            self.bootstrap_peers.to_owned(),
            peer_store_clone,
            task_mng,
        );

        tokio::spawn(async move {
            match disc.start().await {
                Ok(_) => Ok(true),
                Err(err) => {
                    // let m = Msg {
                    //     msg_type: MsgType::SetupFailure,
                    //     msg: "power".into(),
                    // };

                    // let b = task_mng.send(m).await;
                    // println!("11");
                    Err(err)
                }
            }
        });

        let peer_store_clone = peer_store.clone();
        let peer_op = PeerOp::new(
            peer_store_clone,
        );

        tokio::spawn(async move {
            match peer_op.start().await {
                Ok(_) => Ok(true),
                Err(err) => {
                    Err(err)
                }
            }
        });
    }
}
