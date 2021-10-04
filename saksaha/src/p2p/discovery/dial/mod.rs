mod handler;
mod routine;

use routine::Routine;
use crate::{
    msg_err,
    node::task_manager::{MsgKind, TaskManager},
    p2p::{
        address::AddressBook,
        credential::Credential,
        discovery::{dial::handler::HandleResult, whoareyou},
        peer::peer_store::PeerStore,
    },
};
use handler::Handler;
use logger::log;
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
    dial_start_rx: Arc<Mutex<Receiver<usize>>>,
}

impl Dial {
    pub fn new(
        address_book: Arc<AddressBook>,
        peer_store: Arc<PeerStore>,
        disc_port: Option<u16>,
        peer_op_port: u16,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
        dial_start_rx: Arc<Mutex<Receiver<usize>>>,
    ) -> Dial {
        Dial {
            address_book,
            peer_store,
            disc_port,
            peer_op_port,
            task_mng,
            credential,
            dial_start_rx,
        }
    }

    pub async fn start(&self, my_disc_port: u16) {
        let routine = Routine::new(
            self.peer_store.clone(),
            self.credential.clone(),
            self.address_book.clone(),
            self.peer_op_port,
            my_disc_port,
        );

        tokio::spawn(async move {
            log!(DEBUG, "Start disc dialing\n");

            routine.run().await;
        });

        let dial_start_rx = self.dial_start_rx.clone();
        tokio::spawn(async move {
            let mut dial_start_rx = dial_start_rx.lock().await;
            match dial_start_rx.recv().await {
                Some(d) => {
                    println!("123123 {}", d);
                },
                None => {

                },
            };

            // let mut routine_start_rx = self.routine_start_rx.lock().await;
        });
    }

}
