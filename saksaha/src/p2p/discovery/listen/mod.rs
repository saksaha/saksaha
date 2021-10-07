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

pub struct Listen {
    disc_port: Option<u16>,
    peer_op_port: u16,
    peer_store: Arc<PeerStore>,
    task_mng: Arc<TaskManager>,
    credential: Arc<Credential>,
}

impl Listen {
    pub fn new(
        disc_port: Option<u16>,
        peer_op_port: u16,
        peer_store: Arc<PeerStore>,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
    ) -> Listen {
        Listen {
            disc_port,
            peer_op_port,
            peer_store,
            task_mng,
            credential,
        }
    }

    pub async fn start(&self) -> Result<u16> {
        let disc_port = match self.disc_port {
            Some(p) => p,
            None => 0,
        };

        let local_addr = format!("127.0.0.1:{}", disc_port);

        let (tcp_listener, local_addr) =
            match TcpListener::bind(local_addr).await {
                Ok(listener) => match listener.local_addr() {
                    Ok(local_addr) => (listener, local_addr),
                    Err(err) => return Err(err.into()),
                },
                Err(err) => return Err(err.into()),
            };

        log!(
            DEBUG,
            "Successfully started disc listening, addr: {}\n",
            local_addr
        );

        let peer_op_port = self.peer_op_port;
        let routine =
            Routine::new(self.peer_store.clone(), self.credential.clone());

        routine.run(tcp_listener, peer_op_port);

        Ok(local_addr.port())
    }
}

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
