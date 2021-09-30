use super::whoareyou::WhoAreYou;
use crate::{
    common::SakResult,
    err_res, msg_err, msg_errd,
    node::task_manager::{Msg, MsgKind, TaskManager},
    p2p::{
        credential::Credential,
        discovery::whoareyou::{self, WhoAreYouAck},
        peer_store::{Peer, PeerStore},
    },
};
use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use logger::log;
use std::{
    sync::{mpsc::SendError, Arc},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::{Mutex, MutexGuard},
};

pub struct Listen {
    disc_port: u16,
    peer_op_port: u16,
    peer_store: Arc<PeerStore>,
    task_mng: Arc<TaskManager>,
    credential: Arc<Credential>,
}

impl Listen {
    pub fn new(
        disc_port: u16,
        peer_op_port: u16,
        peer_store: Arc<PeerStore>,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
    ) -> Listen {
        Listen {
            disc_port,
            peer_op_port,
            peer_store,
            task_mng,
            credential,
        }
    }

    pub async fn start_listening(&self) {
        let local_addr = format!("127.0.0.1:{}", self.disc_port);
        let task_mng = self.task_mng.clone();

        let (tcp_listener, local_addr) =
            match TcpListener::bind(local_addr).await {
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

                    self.task_mng
                        .send(msg)
                        .await
                        .expect("Fatal message should be delivered");

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
            println!("start loop");
            let mut peer_store = self.peer_store.clone();

            let peer = match peer_store.next().await {
                Some(p) => p,
                None => {
                    tokio::time::sleep(Duration::from_millis(1000)).await;
                    continue;
                }
            };

            let (stream, addr) = match tcp_listener.accept().await {
                Ok(res) => {
                    log!(DEBUG, "Accepted incoming request, addr: {}\n", res.1);
                    res
                }
                Err(err) => {
                    log!(DEBUG, "Error accepting request, err: {}", err);
                    continue;
                }
            };

            tokio::spawn(async move {
                let mut handler = Handler::new(
                    stream,
                    peer,
                    self.credential,
                    self.peer_op_port,
                );

                match handler.run().await {
                    Ok(_) => (),
                    Err(err) => {
                        log!(
                            DEBUG,
                            "Error processing request, addr: {}, err: {}",
                            addr,
                            err
                        );
                    }
                }
            });
        }
    }
}

pub struct Handler {
    stream: TcpStream,
    peer: Arc<Mutex<Peer>>,
    credential: Arc<Credential>,
    peer_op_port: u16,
}

impl Handler {
    pub fn new(
        stream: TcpStream,
        peer: Arc<Mutex<Peer>>,
        credential: Arc<Credential>,
        peer_op_port: u16,
    ) -> Handler {
        Handler {
            stream,
            peer,
            credential,
            peer_op_port,
        }
    }

    pub async fn run(&mut self) -> SakResult<bool> {
        let way = match WhoAreYou::parse(&mut self.stream).await {
            Ok(w) => w,
            Err(err) => {
                return err_res!(
                    "Error parsing who are you request, err: {}",
                    err
                );
            }
        };

        println!("received: {:?}, {}", way.sig, way.peer_op_port);

        let secret_key = &self.credential.secret_key;
        let signing_key = SigningKey::from(secret_key);
        let sig: Signature = signing_key.sign(whoareyou::MESSAGE);

        let way_ack = WhoAreYouAck::new(sig, self.peer_op_port);

        self.stream.write_all(b"hello\n").await;

        Ok(true)
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

        // let listen =
        //     Listen::new(disc_port, peer_store.clone(), task_mng.clone());
        // let listen2 =
        //     Listen::new(disc_port, peer_store.clone(), task_mng.clone());

        // let h2 = tokio::spawn(async move {
        //     listen.start_listening().await;
        //     println!("h3");
        // });

        // let h3 = tokio::spawn(async move {
        //     listen2.start_listening().await;
        //     return true;
        // });

        // tokio::select! {
        //     _ = h2 => (),
        //     res = h3 => {
        //         assert!(res.unwrap(),
        //             "Listen should fail while attempting to use the taken port")
        //     },
        // }
    }
}
