mod handler;
mod routine;

use crate::{
    common::Result,
    msg_err,
    node::task_manager::{MsgKind, TaskManager},
    p2p::{credential::Credential, peer::peer_store::PeerStore},
};
use handler::Handler;
use logger::log;
use routine::Routine;
use std::sync::Arc;
use tokio::{net::TcpListener, sync::Mutex};

pub struct Listen {}

impl Listen {
    pub fn new() -> Listen {
        Listen {}
    }

    pub async fn start(
        &self,
        disc_listener: TcpListener,
        peer_op_port: u16,
        peer_store: Arc<PeerStore>,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
    ) -> Result<()> {
        let routine = Routine::new();
        routine.run(disc_listener, peer_op_port, peer_store, credential);

        Ok(())
    }
}

/// currently not used
#[cfg(test)]
mod test {
    // use super::*;

    #[tokio::test]
    async fn test_create_new_disc() {
        // let peer_store = Arc::new(PeerStore::new(10));
        // let task_mng = Arc::new(TaskManager::new());
        // let disc_port = 13131;

        // let listen =
        //     Listen::new(disc_port, peer_store.clone(), task_mng.clone());
        // let listen2 =
        //     Listen::new(disc_port, peer_store.clone(), task_mng.clone());

        // let h2 = tokio::spawn(async move {
        //     listen.start_listening().await;
        //     println!("h3");
        // });

        // let h3 = tokio::spawn(async move {
        //     listen2.start_listening().await;
        //     return true;
        // });

        // tokio::select! {
        //     _ = h2 => (),
        //     res = h3 => {
        //         assert!(res.unwrap(),
        //             "Listen should fail while attempting to use the taken port")
        //     },
        // }
    }
}
