mod handler;
mod routine;

use crate::{
    common::Result,
    p2p::{credential::Credential, peer::peer_store::PeerStore},
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

pub struct Dialer {}

impl Dialer {
    pub fn new() -> Dialer {
        Dialer {}
    }

    pub async fn start(
        &self,
        my_disc_port: u16,
        peer_store: Arc<PeerStore>,
        peer_op_port: u16,
        credential: Arc<Credential>,
    ) -> Result<()> {
        let routine = Arc::new(Routine::new(
            peer_store.clone(),
            credential.clone(),
            peer_op_port,
            my_disc_port,
        ));

        let routine_clone = routine.clone();
        routine_clone.run();

        Ok(())

        // let routine_clone = routine.clone();
        // let disc_wakeup_rx = disc_wakeup_rx.clone();
        // tokio::spawn(async move {
        //     loop {
        //         let mut disc_wakeup_rx = disc_wakeup_rx.lock().await;

        //         match disc_wakeup_rx.recv().await {
        //             Some(_) => {
        //                 routine_clone.wakeup().await;
        //             }
        //             None => {
        //                 let msg = msg_err!(
        //                     Kind::SetupFailure,
        //                     "Cannot receive disc dial wakeup msg, \
        //                     is channel closed?",
        //                 );

        //                 // task_mng.send(msg).await;

        //                 tokio::time::sleep(Duration::from_millis(1000)).await;
        //             }
        //         };
        //     }
        // });
    }
}
