use super::{whoareyou::WhoAreYou, Disc};
use crate::{
    common::SakResult,
    crypto::Crypto,
    err_res,
    node::task_manager::TaskManager,
    p2p::{
        address::AddressBook,
        credential::Credential,
        discovery::whoareyou::{self, WhoAreYouAck},
        peer::{Peer, peer_store::PeerStore},
    },
};
use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    elliptic_curve::sec1::ToEncodedPoint,
    EncodedPoint,
};
use logger::log;
use std::{
    sync::Arc,
    thread,
    time::{Duration, SystemTime},
};
use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    sync::{Mutex, MutexGuard},
    time,
};

pub struct Dial {
    pub address_book: Arc<AddressBook>,
    pub peer_store: Arc<PeerStore>,
    disc_port: u16,
    peer_op_port: u16,
    task_mng: Arc<TaskManager>,
    credential: Arc<Credential>,
}

impl Dial {
    pub fn new(
        address_book: Arc<AddressBook>,
        peer_store: Arc<PeerStore>,
        disc_port: u16,
        peer_op_port: u16,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
    ) -> Dial {
        Dial {
            address_book,
            peer_store,
            disc_port,
            peer_op_port,
            task_mng,
            credential,
        }
    }

    pub async fn start_dialing(&self) {
        let my_disc_endpoint = format!("127.0.0.1:{}", self.disc_port);

        loop {
            let start = SystemTime::now();

            let (addr, idx) = match self.address_book.next().await {
                Some(a) => a,
                None => {
                    println!("Addr not available");
                    return;
                    time::sleep(Duration::from_millis(1000)).await;
                    continue;
                }
            };
            let addr = addr.lock().await;

            let peer = match self.peer_store.next().await {
                Some(p) => p,
                None => {
                    println!("Peer not available");
                    time::sleep(Duration::from_millis(1000)).await;
                    continue;
                }
            };

            if addr.endpoint == my_disc_endpoint {
                match self.address_book.remove(idx).await {
                    Ok(_) => (),
                    Err(err) => {
                        log!(
                            DEBUG,
                            "Error removing address, idx: {}, endpoint: {}",
                            idx,
                            addr.endpoint
                        );
                    }
                }
                continue;
            }

            let stream =
                match TcpStream::connect(addr.endpoint.to_owned()).await {
                    Ok(s) => {
                        log!(
                            DEBUG,
                            "Successfully connected to endpoint, {}\n",
                            addr.endpoint
                        );
                        s
                    }
                    Err(err) => {
                        log!(
                            DEBUG,
                            "Error connecting to addr, {:?}, err: {}",
                            addr,
                            err
                        );
                        continue;
                    }
                };

            let credential = self.credential.clone();

            let mut handler =
                Handler::new(stream, peer, credential, self.peer_op_port);

            match handler.run().await {
                Ok(_) => (),
                Err(err) => {
                    log!(
                        DEBUG,
                        "Error processing request, endpoint: {}, err: {}\n",
                        addr.endpoint,
                        err,
                    );
                }
            }

            return;

            tokio::time::sleep(Duration::new(1, 0)).await;
            match start.elapsed() {
                Ok(_) => (),
                Err(err) => {
                    log!(DEBUG, "Error sleeping the duration, err: {}", err);
                }
            }
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
            stream: stream,
            peer,
            credential,
            peer_op_port,
        }
    }

    pub async fn run(&mut self) -> SakResult<bool> {
        let secret_key = &self.credential.secret_key;
        let signing_key = SigningKey::from(secret_key);
        let sig: Signature = signing_key.sign(whoareyou::MESSAGE);

        let way = WhoAreYou::new(
            sig,
            self.peer_op_port,
            self.credential.public_key_bytes,
        );

        let buf = match way.to_bytes() {
            Ok(b) => b,
            Err(err) => {
                return err_res!(
                    "Error creating WhoAreYou request, err: {}",
                    err
                );
            }
        };

        match self.stream.write_all(&buf).await {
            Ok(_) => (),
            Err(err) => {
                return err_res!(
                    "Error sending the whoAreYou buffer, err: {}",
                    err
                );
            }
        }

        let way_ack = match WhoAreYouAck::parse(&mut self.stream).await {
            Ok(w) => w,
            Err(err) => {
                return err_res!("Cannot process WhoAreyouAck, err: {}", err);
            }
        };

        println!("dial received way_ack: {:?}", way_ack.to_bytes());

        let verifying_key = match Crypto::convert_public_key_to_verifying_key(
            way_ack.way.public_key_bytes,
        ) {
            Ok(v) => v,
            Err(err) => {
                return err_res!("Error creating verifying key, err: {}", err);
            }
        };
        let sig = way_ack.way.sig;

        match verifying_key.verify(whoareyou::MESSAGE, &sig) {
            Ok(_) => (),
            Err(err) => {
                return err_res!("Signature is invalid, err: {}", err);
            }
        };

        let peer = self.peer.lock().await;

        Ok(true)
    }
}
