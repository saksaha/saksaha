mod dialer;
mod listener;
mod status;
mod whoareyou;

use self::listener::Listener;
use crate::{
    common::{Error, Result},
    p2p::{credential::Credential, peer::peer_store::PeerStore},
};
use dialer::Dialer;
use futures::stream::{FuturesUnordered};
pub use status::Status;
use std::{collections::VecDeque, sync::Arc};
use tokio::{sync::{mpsc::{self, Sender}}};

pub struct Disc {}

impl Disc {
    pub fn new() -> Disc {
        Disc {}
    }

    pub async fn start(
        &self,
        port: Option<u16>,
        p2p_listener_port: u16,
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
        bootstrap_urls: Option<Vec<String>>,
    ) -> Status<Error> {
        let listener = Listener::new();
        let listener_port = match listener
            .start(
                port,
                p2p_listener_port,
                peer_store.clone(),
                credential.clone(),
            )
            .await
        {
            listener::Status::Launched(port) => port,
            listener::Status::SetupFailed(err) => {
                return Status::SetupFailed(err)
            }
        };

        let ff = || async {
            println!("333");
        };

        let (tasks_tx, mut tasks_rx) = mpsc::channel(10);
        // tx.send(ff).await;
        // tx.send(ff).await;

        // tasks_tx.send(async {

        // });
        let a = tokio::spawn(async {

        });

        a.await;

        println!("11");

        // tokio::spawn(async move {
        //     loop {
        //         let f = rx.recv().await.unwrap();
        //         f().await;
        //     }
        // });

        // println!("22");

        // tx.send(ff).await;
        // tx.send(ff).await;
        // tx.send(ff).await;
        // tx.send(ff).await;

        // loop {
        //     match aa.next().await {
        //         Some(result) => {
        //           println!("    finished future [{}]", result);
        //           if cnt < 20 {
        //             workers.push( random_sleep(cnt) );
        //           }
        //         },
        //         None => {
        //           println!("Done!");
        //           break;

        // }


        // let ff = || {
        //     println!("33");
        // };

        // let mut v = vec!();
        // v.push(f);
        // v.push(f);

        // let mut vv = vec!();
        // vv.push(ff);
        // vv.push(ff);

        // let mut tasks = vec!();
        // tasks.push(async {
        //     println!("3");
        // });

        // let q = FuturesUnordered::new();
        // q.push(Box::pin(async {
        //     println!("f");
        // }));

        // for a in q.iter() {

        // }

        // q.


        // tasks.
        // loop {
        // }


        // loop {
        //     tasks
        // }


        // let dialer = Dialer::new();
        // match dialer
        //     .start(
        //         listener_port,
        //         peer_store.clone(),
        //         p2p_listener_port,
        //         credential.clone(),
        //     )
        //     .await
        // {
        //     Ok(_) => (),
        //     Err(err) => return Status::SetupFailed(err),
        // };

        Status::Launched
    }
}
