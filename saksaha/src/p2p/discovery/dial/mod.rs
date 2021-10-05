mod handler;
mod routine;

use crate::{
    msg_err, msg_errd,
    node::task_manager::{MsgKind, TaskManager},
    p2p::{
        address::address_book::AddressBook, credential::Credential,
        discovery::whoareyou, peer::peer_store::PeerStore,
    },
};
use handler::Handler;
use logger::log;
use routine::Routine;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

pub struct Dial {
    pub address_book: Arc<AddressBook>,
    pub peer_store: Arc<PeerStore>,
    peer_op_port: u16,
    task_mng: Arc<TaskManager>,
    credential: Arc<Credential>,
    disc_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
    peer_op_wakeup_tx: Arc<Sender<usize>>,
}

impl Dial {
    pub fn new(
        address_book: Arc<AddressBook>,
        peer_store: Arc<PeerStore>,
        peer_op_port: u16,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
        disc_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
        peer_op_wakeup_tx: Arc<Sender<usize>>,
    ) -> Dial {
        Dial {
            address_book,
            peer_store,
            peer_op_port,
            task_mng,
            credential,
            disc_wakeup_rx,
            peer_op_wakeup_tx,
        }
    }

    pub async fn start(&self, my_disc_port: u16) {
        let task_mng = self.task_mng.clone();

        let routine = Arc::new(Routine::new(
            self.peer_store.clone(),
            self.credential.clone(),
            self.address_book.clone(),
            self.peer_op_port,
            my_disc_port,
            self.peer_op_wakeup_tx.clone(),
        ));

        let routine_clone = routine.clone();
        routine_clone.run();

        let routine_clone = routine.clone();
        let disc_wakeup_rx = self.disc_wakeup_rx.clone();
        tokio::spawn(async move {
            loop {
                let mut disc_wakeup_rx = disc_wakeup_rx.lock().await;
                match disc_wakeup_rx.recv().await {
                    Some(_) => {
                        routine_clone.wakeup().await;
                    }
                    None => {
                        let msg = msg_errd!(
                            "Cannot receive dial wakeup msg, \
                            is channel closed?",
                        );

                        task_mng.send(msg).await;
                    }
                };
            }
        });
    }
}
