use super::whoareyou::WhoAreYou;
use crate::{
    common::{testenv::run_test, SakResult},
    err_res,
    p2p::peer_store::{Peer, PeerStore},
};
use logger::log;
use std::sync::Arc;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::{TcpListener, TcpStream},
    sync::{Mutex, MutexGuard},
};

pub struct Listen {
    pub disc_port: usize,
    pub peer_store: Arc<Mutex<PeerStore>>,
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
    pub fn new(disc_port: usize, peer_store: Arc<Mutex<PeerStore>>) -> Listen {
        Listen {
            disc_port,
            peer_store,
        }
    }

    pub async fn start_listening(&self) -> SakResult<bool> {
        let local_addr = format!("127.0.0.1:{}", self.disc_port);

        let (tcp_listener, local_addr) =
            match TcpListener::bind(local_addr).await {
                Ok(l) => {
                    let local_addr = match l.local_addr() {
                        Ok(a) => a,
                        Err(err) => {
                            return err_res!(
                                "Error getting the local address of \
                                disc listener, err: {}",
                                err
                            );
                        }
                    };

                    (l, local_addr)
                }
                Err(_) => {
                    return err_res!("Error start listeneing");
                }
            };

        log!(
            DEBUG,
            "Successfully started disc listening, addr: {}\n",
            local_addr
        );

        return self.run_loop(tcp_listener).await;
    }

    pub async fn run_loop(&self, tcp_listener: TcpListener) -> SakResult<bool> {
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
                    return err_res!("Error accepting a request, err: {}", err);
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
