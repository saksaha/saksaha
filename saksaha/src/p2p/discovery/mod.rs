mod dial;
mod listen;
mod whoareyou;

use self::listen::Listen;
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

    pub async fn start(self) -> SakResult<bool> {
        let peer_store = self.peer_store.clone();
        let listen = listen::Listen::new(self.disc_port, peer_store);

        tokio::spawn(async move {
            match listen.start_listening().await {
                Ok(_) => Ok(1),
                Err(err) => {
                    return err_res!(
                        "Error start disc listening, err: {}",
                        err
                    );
                }
            }
        });

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

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_create_new_disc() {
        let peer_store = Arc::new(Mutex::new(PeerStore::new(10)));
        let disc_port = 61232;
        let bootstrap_peers = Some(vec![String::from("test")]);
        let task_mng = Arc::new(TaskManager::new());
        let disc = Disc::new(disc_port, bootstrap_peers, peer_store, task_mng);
        let _result = match disc.start().await {
            Ok(res) => assert!(res),
            Err(_err) => {
                panic!("Test Failed {}", _err);
            }
        };
    }

    #[tokio::test]
    async fn test_start_listening() {
        let peer_store = Arc::new(Mutex::new(PeerStore::new(12)));
        let disc_port = 39450;
        let listen = listen::Listen::new(disc_port, peer_store);
        let _result = tokio::spawn(async move {
            match listen.start_listening().await {
                Ok(_) => (),
                Err(_err) => {
                    panic!("Test Failed {}", _err);
                }
            }
        });
    }
}
