use super::whoareyou::WhoAreYou;
use crate::{
    common::SakResult,
    err_res, msg_err, msg_errd,
    node::task_manager::{Msg, MsgKind, TaskManager},
    p2p::peer_store::{Peer, PeerStore},
};
use logger::log;
use std::sync::{Arc, mpsc::SendError};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::{Mutex, MutexGuard},
};

pub struct Listen {
    disc_port: usize,
    peer_store: Arc<PeerStore>,
    task_mng: Arc<TaskManager>,
}

pub struct Handler<'a> {
    stream: TcpStream,
    peer: MutexGuard<'a, Peer>,
}

impl<'a> Handler<'a> {
    pub async fn run(&mut self) -> SakResult<bool> {
        let way = WhoAreYou::parse(&mut self.stream).await;
        Ok(true)
    }
}

impl Listen {
    pub fn new(
        disc_port: usize,
        peer_store: Arc<PeerStore>,
        task_mng: Arc<TaskManager>,
    ) -> Listen {
        Listen {
            disc_port,
            peer_store,
            task_mng,
        }
    }

    pub async fn start_listening(&self) {
        let local_addr = format!("127.0.0.1:{}", self.disc_port);
        let task_mng = self.task_mng.clone();

        let (tcp_listener, local_addr) = match TcpListener::bind(local_addr)
            .await
        {
            Ok(l) => {
                let local_addr = match l.local_addr() {
                    Ok(a) => a,
                    Err(err) => {
                        let msg = msg_err!(
                            MsgKind::SetupFailure,
                            "Error getting the local addr, disc listen, {}",
                            err
                        );

                        if let Err(err) = task_mng.send(msg).await {
                            log!(
                                DEBUG,
                                "Error sending a msg to task manager, err: {}",
                                err
                            );

                            self.task_mng.shutdown_program();
                        }
                        unreachable!()
                    }
                };

                (l, local_addr)
            }
            Err(err) => {
                log!(
                    DEBUG,
                    "Error getting the endpoint, disc listen, {}\n",
                    err
                );

                let msg = msg_err!(
                    MsgKind::SetupFailure,
                    "Error getting the endpoint, disc listen, {}",
                    err
                );

                match self.task_mng.send(msg).await {
                    Ok(_) => (),
                    Err(err) => {
                        log!(
                            DEBUG,
                            "Error sending a msg to task manager, err: {}",
                            err
                        );

                        self.task_mng.shutdown_program();
                    }
                }

                return;
            }
        };

        log!(
            DEBUG,
            "Successfully started disc listening, addr: {}\n",
            local_addr
        );

        self.run_loop(tcp_listener).await;

        unreachable!();
    }

    pub async fn run_loop(&self, tcp_listener: TcpListener) {
        loop {
            // let mut peer_store = self.peer_store.lock().await;

            // let idx = match peer_store.reserve_slot() {
            //     Some(i) => i,
            //     None => {
            //         // TODO: need to sleep for a while until making new attempts
            //         continue;
            //     }
            // };

            // let (stream, addr) = match tcp_listener.accept().await {
            //     Ok(res) => res,
            //     Err(err) => {
            //         log!(DEBUG, "Error accepting request, err: {}", err);
            //         continue;
            //     }
            // };

            // log!(DEBUG, "Accepted incoming request, addr: {}\n", addr);

            // let peer_store = self.peer_store.clone();

            // tokio::spawn(async move {
            //     let peer_store = peer_store.lock().await;

            //     let peer = if let Some(p) = peer_store.slots.get(idx) {
            //         if let Ok(p) = p.try_lock() {
            //             p
            //         } else {
            //             log!(
            //                 DEBUG,
            //                 "Error getting mutex, something \
            //                 might be wrong, idx: {}",
            //                 idx
            //             );
            //             return;
            //         }
            //     } else {
            //         return;
            //     };

            //     let mut h = Handler { stream, peer };
            //     h.run().await;
            // });
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_create_new_disc() {
        let peer_store = Arc::new(PeerStore::new(10));
        let task_mng = Arc::new(TaskManager::new());

        let disc_port = 13131;
        let listen =
            Listen::new(disc_port, peer_store.clone(), task_mng.clone());
        let listen2 =
            Listen::new(disc_port, peer_store.clone(), task_mng.clone());

        let h2 = tokio::spawn(async move {
            listen.start_listening().await;
            println!("h3");
        });

        let h3 = tokio::spawn(async move {
            listen2.start_listening().await;
            return true;
        });

        tokio::select! {
            _ = h2 => (),
            res = h3 => {
                assert!(res.unwrap(),
                    "Listen should fail while attempting to use the taken port")
            },
        }
    }
}
