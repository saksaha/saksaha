use super::{whoareyou::WhoAreYou, Disc};
use crate::{common::SakResult, err_res, node::task_manager::TaskManager, p2p::{address::AddressBook, credential::Credential, discovery::whoareyou::{self, WhoAreYouAck}, peer_store::{Peer, PeerStore}}};
use k256::ecdsa::{signature::Signer, Signature, SigningKey};
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

            println!("33 {:?}, {:?}", addr, peer);
            if addr.endpoint != my_disc_endpoint {
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
                let mut handler = Handler::new(
                    stream,
                    peer,
                    credential,
                    self.disc_port,
                    self.peer_op_port,
                );
                handler.run().await;
                // Handler::run(stream, peer, credential).await;

                // let (mut rd, mut wr) = io::split(stream);

                // println!("31355");

                // wr.write_all(b"power\n").await.unwrap();

                // let mut buf = vec![0; 128];

                // println!("313");

                // loop {
                //     let n = rd.read(&mut buf).await.unwrap();

                //     if n == 0 {
                //         break;
                //     }

                //     println!("GOT {:?}", &buf[..n]);
                // }

                // println!("31344");

                // let h = Handler::new(stream);
                // h.run();
            } else {
                println!("!313");
                match self.address_book.remove(idx).await {
                    Ok(_) => (),
                    Err(err) => {
                        println!("err: {}", err);
                    }
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
    disc_port: u16,
    peer_op_port: u16,
}

impl Handler {
    pub fn new(
        stream: TcpStream,
        peer: Arc<Mutex<Peer>>,
        credential: Arc<Credential>,
        disc_port: u16,
        peer_op_port: u16,
    ) -> Handler {
        Handler {
            stream: stream,
            peer,
            credential,
            disc_port,
            peer_op_port,
        }
    }

    pub async fn run(&mut self) -> SakResult<bool> {
        let secret_key = &self.credential.secret_key;
        let signing_key = SigningKey::from(secret_key);
        let sig: Signature = signing_key.sign(whoareyou::MESSAGE);

        let way = WhoAreYou {
            sig,
            peer_op_port: self.peer_op_port,
        };

        let buf = match way.to_bytes() {
            Ok(b) => b,
            Err(err) => {
                return err_res!(
                    "Error creating WhoAreYou request, err: {}",
                    err
                );
            }
        };

        println!("1, {:?}", buf);

        match self.stream.write_all(&buf).await {
            Ok(_) => (),
            Err(err) => {
                return err_res!(
                    "Error sending the whoAreYou buffer, err: {}",
                    err
                );
            }
        }

        let way = WhoAreYouAck::parse(&mut self.stream).await;


        // self.stream.read_buf()

        Ok(true)
    }
}
