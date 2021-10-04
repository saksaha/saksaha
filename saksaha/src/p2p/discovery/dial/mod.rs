mod handler;
mod routine;

use crate::{
    msg_err, msg_errd,
    node::task_manager::{MsgKind, TaskManager},
    p2p::{
        address::AddressBook,
        credential::Credential,
        discovery::{whoareyou},
        peer::peer_store::PeerStore,
    },
};
use handler::Handler;
use logger::log;
use routine::Routine;
use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};
use tokio::sync::{mpsc::Receiver, Mutex};

pub struct Dial {
    pub address_book: Arc<AddressBook>,
    pub peer_store: Arc<PeerStore>,
    disc_port: Option<u16>,
    peer_op_port: u16,
    task_mng: Arc<TaskManager>,
    credential: Arc<Credential>,
    dial_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
}

impl Dial {
    pub fn new(
        address_book: Arc<AddressBook>,
        peer_store: Arc<PeerStore>,
        disc_port: Option<u16>,
        peer_op_port: u16,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
        dial_wakeup_rx: Arc<Mutex<Receiver<usize>>>,
    ) -> Dial {
        Dial {
            address_book,
            peer_store,
            disc_port,
            peer_op_port,
            task_mng,
            credential,
            dial_wakeup_rx,
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
        ));

        let routine_clone = routine.clone();
        routine_clone.run();

        let routine_clone = routine.clone();
        let dial_wakeup_rx = self.dial_wakeup_rx.clone();
        tokio::spawn(async move {
            let mut dial_wakeup_rx = dial_wakeup_rx.lock().await;

            match dial_wakeup_rx.recv().await {
                Some(_) => {
                    routine_clone.wakeup().await;
                }
                None => {
                    let msg = msg_errd!(
                        "Cannot receive dial wakeup msg, is channel closed?",
                    );

                    task_mng.send(msg).await;
                }
            };
        });
    }
}
