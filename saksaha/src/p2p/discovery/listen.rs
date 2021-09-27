use crate::{
    common::SakResult,
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
    pub peer_store: Arc<PeerStore>,
}

pub struct Handler<'a> {
    stream: TcpStream,
    peer: MutexGuard<'a, Peer>,
}

impl<'a> Handler<'a> {
    pub async fn run(&mut self) {
        let mut buf = vec![0; 1024];

        log!(DEBUG, "Parsing msg, peer id: {}\n", self.peer.i);

        loop {
            let n = self.stream.read(&mut buf).await.unwrap();

            if n == 0 {
                return;
            }

            println!("{:?}", buf);
        }
    }
}

impl Listen {
    pub async fn start_listening(self) -> SakResult<bool> {
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

        loop {
            let idx = match self.peer_store.reserve_slot() {
                Some(i) => i,
                None => {
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

                let mut h = Handler {
                    stream: stream,
                    peer,
                };
                h.run().await;
            });
        }

        Ok(true)
    }

    async fn handle_connection(mut stream: tokio::net::TcpStream) {
        println!("{:?}", stream);

        let mut buf = vec![0; 1024];

        loop {
            let n = stream.read(&mut buf).await.unwrap();

            if n == 0 {
                return;
            }

            println!("{:?}", buf);
        }
    }
}
