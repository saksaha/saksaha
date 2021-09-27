use crate::{
    common::SakResult,
    err_res,
    p2p::peer_store::{Peer, PeerStore},
};
use logger::log;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub struct Listen {
    pub disc_port: usize,
    pub peer_store: Arc<PeerStore>,
}

pub struct Handler {
    stream: TcpStream,
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
            let slots = self.peer_store.slots.lock().await;
            let cap = self.peer_store.capacity;
            for i in 0..cap {
                let idx = self.peer_store.curr_idx + i % cap;
                let peer = match slots.get(idx) {
                    Some(p) => p,
                    None => {
                        return err_res!(
                            "Error getting peer in the slot, \
                            it may have been accidentally removed"
                        );
                    }
                };

            let mut peer = match peer.try_lock() {
                Ok(p) => {
                    log!(DEBUG, "Acquired a peer, at idx: {}\n", idx);
                    p
                }
                Err(_) => {
                    continue;
                }
            };

            println!("55, {}", peer.i);

            let (stream, addr) = match tcp_listener.accept().await {
                Ok(res) => res,
                Err(err) => {
                    return err_res!("Error accepting a request, err: {}", err);
                }
            };

            println!("new: {}, {}", addr, peer.i);

            let h = Handler { stream };
        }




            // self.peer_store.take_empty_slot(|peer| {
            //     Box::pin(async move {
            //         tokio::spawn(async {
            //             println!("peer: {}", peer.i);
            //         });
            //     })

            //     // h;
            //     // let h = Handler { stream };
            //     // let b = Handler { stream, };
            // }).await;

            // tokio::spawn(async move {
            //     stream;
            //     // clone.take_empty_slot(|| async move {
            //     //     // Listen::handle_connection(stream).await;
            //     //     // return false;
            //     //     return false;
            //     // });

            // });
        }

        // loop {
        //     let (stream, addr) = match tcp_listener.accept().await {
        //         Ok(res) => res,
        //         Err(err) => {
        //             return err_res!("Error accepting a request, err: {}", err);
        //         }
        //     };

        //     log!(DEBUG, "New incoming disc connection, addr: {}\n", addr);

        // tokio::spawn(async move {
        //     Listen::handle_connection(stream).await;
        // });
        // }

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
