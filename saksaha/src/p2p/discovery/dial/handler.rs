use super::{whoareyou::WhoAreYou, Disc};
use crate::{
    common::SakResult,
    crypto::Crypto,
    err_res,
    p2p::{
        address::AddressBook,
        credential::Credential,
        discovery::whoareyou::{self, WhoAreYouAck},
        peer::{peer_store::PeerStore, Peer, PeerStatus},
    },
};
use k256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey, VerifyingKey,
};
use logger::log;
use std::{sync::Arc, time::Duration};
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex, time};

pub enum HandleResult {
    AddressNotFound,

    LocalAddrIdentical,

    Success,
}

pub struct Handler {
    peer: Arc<Mutex<Peer>>,
    credential: Arc<Credential>,
    peer_op_port: u16,
    address_book: Arc<AddressBook>,
    my_disc_endpoint: String,
}

impl Handler {
    pub fn new(
        peer: Arc<Mutex<Peer>>,
        credential: Arc<Credential>,
        peer_op_port: u16,
        address_book: Arc<AddressBook>,
        my_disc_endpoint: String,
    ) -> Handler {
        Handler {
            peer,
            credential,
            peer_op_port,
            address_book,
            my_disc_endpoint,
        }
    }

    pub async fn run(&mut self) -> SakResult<HandleResult> {
        let (addr, idx) = match self.address_book.next().await {
            Some(a) => a,
            None => {
                log!(DEBUG, "Cannot acquire next address\n");

                return Ok(HandleResult::AddressNotFound);
            }
        };

        let addr = addr.lock().await;

        if addr.endpoint == self.my_disc_endpoint {
            log!(
                DEBUG,
                "Discarding dial request, endpoint to local, addr: {}",
                addr.endpoint
            );

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
            return Ok(HandleResult::LocalAddrIdentical);
        }

        let mut stream =
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
                    return err_res!(
                        "Error connecting to addr: {:?}, err: {}",
                        addr,
                        err
                    );
                }
            };

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

        match stream.write_all(&buf).await {
            Ok(_) => (),
            Err(err) => {
                return err_res!(
                    "Error sending the whoAreYou buffer, err: {}",
                    err
                );
            }
        }

        let way_ack = match WhoAreYouAck::parse(&mut stream).await {
            Ok(w) => w,
            Err(err) => {
                return err_res!("Cannot process WhoAreyouAck, err: {}", err);
            }
        };

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

        let mut peer = self.peer.lock().await;
        peer.status = PeerStatus::Discovered;
        peer.endpoint = addr.endpoint.to_owned();
        peer.peer_id = way_ack.way.get_peer_id();

        println!("peer: {:?}", peer);

        tokio::spawn(async move {
            println!("Start synchroize");
        });

        Ok(HandleResult::Success)
    }
}
