mod dial;
mod listen;
mod whoareyou;

use super::peer_store::PeerStore;
use crate::{common::SakResult, err_res, node::task_manager::TaskManager};
use logger::log;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Disc {
    disc_port: usize,
    bootstrap_peers: Option<Vec<String>>,
    peer_store: Arc<Mutex<PeerStore>>,
    task_mng: Arc<TaskManager>,
}

impl Disc {
    pub fn new(
        disc_port: usize,
        bootstrap_peers: Option<Vec<String>>,
        peer_store: Arc<Mutex<PeerStore>>,
        task_mng: Arc<TaskManager>,
    ) -> Self {
        Disc {
            disc_port,
            bootstrap_peers,
            peer_store,
            task_mng,
        }
    }

    pub async fn start(self, ) -> SakResult<bool> {
        let peer_store = self.peer_store.clone();
        let listen = listen::Listen::new(self.disc_port, peer_store);

        let listen_handle = tokio::spawn(async move {
            match listen.start_listening().await {
                Ok(_) => Ok(1),
                Err(err) => {
                    return err_res!("Error start disc listening, err: {}", err);
                }
            }
        });

        match listen_handle.await {
            Ok(handle) => {
                match handle {
                    Ok(_) => (),
                    Err(err) => {
                        return err_res!("Disc listen thread has returned \
                            with error, err: {}", err);
                    }
                }
            }
            Err(err) => {
                return err_res!("Error spawning disc listen, err: {}", err);
            }
        }

        let peer_store = self.peer_store.clone();
        let dialer = dial::Dial::new(self.bootstrap_peers, peer_store);

        tokio::spawn(async move {
            match dialer.start_dialing().await {
                Ok(_) => Ok(()),
                Err(err) => {
                    return err_res!("Error start disc dialing, err: {}", err);
                }
            }
        });

        Ok(true)
    }
}

#[tokio::test]
async fn aaa() {
    let peer_store = Arc::new(Mutex::new(PeerStore::new(10)));
    let disc_port = 35518;
    let bootstrap_peers = Some(vec!(String::from("test")));
    let task_mng = Arc::new(TaskManager::new());

    let disc = Disc::new(disc_port, bootstrap_peers, peer_store, task_mng);

    match disc.start().await {
        Ok(_) => (),
        Err(err) => {
            println!("555555");
        }
    }

    // let m = Msg {
    //     msg_type: MsgType::SetupFailure,
    //     msg: "power".into(),
    // };

    // let a = t.clone();

    // let b = tokio::spawn(async move {
    //     t.send(m).await;
    // });

    // tokio::join!(b);

    // a.receive().await;

    println!("3333333333");
}
