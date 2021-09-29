use super::whoareyou::WhoAreYou;
use crate::{common::{testenv::run_test, SakResult}, err_res, msg_err, node::task_manager::{Msg, MsgKind, TaskManager}, p2p::peer_store::{Peer, PeerStore}};
use logger::log;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::{TcpListener, TcpStream},
    sync::{Mutex, MutexGuard},
};

pub struct Listen {
    disc_port: usize,
    peer_store: Arc<Mutex<PeerStore>>,
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
        peer_store: Arc<Mutex<PeerStore>>,
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
                        let msg =
                        Msg::new(err.to_string(), MsgKind::SetupFailure);
                        task_mng.send(msg).await;

                        return;
                    }
                };

                (l, local_addr)
            }
            Err(err) => {
                println!("ee");
                let msg = msg_err!(MsgKind::SetupFailure, "Err1, {}", err);
                self.task_mng.send(msg).await;

                println!("ee2");
                return;
            }
        };

        log!(
            DEBUG,
            "Successfully started disc listening, addr: {}\n",
            local_addr
        );

        self.run_loop(tcp_listener).await;
    }

    pub async fn run_loop(&self, tcp_listener: TcpListener) {
        loop {
            let mut peer_store = self.peer_store.lock().await;

            let idx = match peer_store.reserve_slot() {
                Some(i) => i,
                None => {
                    // TODO: need to sleep for a while until making new attempts
                    continue;
                }
            };

            let (stream, addr) = match tcp_listener.accept().await {
                Ok(res) => res,
                Err(err) => {
                    return;
                    // return err_res!("Error accepting a request, err: {}", err);
                }
            };

            log!(DEBUG, "Accepted incoming request, addr: {}\n", addr);

            let peer_store = self.peer_store.clone();

            tokio::spawn(async move {
                let peer_store = peer_store.lock().await;

                let peer = if let Some(p) = peer_store.slots.get(idx) {
                    if let Ok(p) = p.try_lock() {
                        p
                    } else {
                        log!(
                            DEBUG,
                            "Error getting mutex, something \
                            might be wrong, idx: {}",
                            idx
                        );
                        return;
                    }
                } else {
                    return;
                };

                let mut h = Handler { stream, peer };
                h.run().await;
            });
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_create_new_disc() {
        println!("333333333");

        let peer_store = Arc::new(Mutex::new(PeerStore::new(10)));
        let task_mng = Arc::new(TaskManager::new());

        let disc_port = 13131;
        let listen = Listen::new(disc_port, peer_store.clone(), task_mng.clone());

        let listen2 = Listen::new(disc_port, peer_store.clone(), task_mng.clone());

        let h1 = tokio::spawn(async move {
            task_mng.clone().receive().await;
            println!("h1");
        });

        let h2 = tokio::spawn(async move {
            listen.start_listening().await;

            println!("h2");
        });

        let h3 = tokio::spawn(async move {
            listen2.start_listening().await;
            println!("h3");
        });

        println!("#33");



        // tokio::select!(
        //     (_) = h1 => {
        //         println!("h1");
        //     },
        //     (_) = h2 => {
        //         println!("h2");
        //     },
        //     (_) = task_mng.clone().receive() => {
        //         println!("receive");
        //     }
        // );

        // let queue = task_mng.msg_queue.lock().await;


        // for elem in queue.iter() {
        //     println!("11, {}", elem.label);
        // }

        println!("4444");

        // task_mng.receive().await;


        // succeed
    }

    #[tokio::test]
    async fn test_start_listening() {
        // let peer_store = Arc::new(Mutex::new(PeerStore::new(12)));
        // let disc_port = 39450;
        // let task_mng = Arc::new(TaskManager::new());

        // let listen = listen::Listen::new(disc_port, peer_store, task_mng);
        // listen.start_listening().await;
    }
}
