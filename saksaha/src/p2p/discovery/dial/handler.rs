use super::whoareyou::WhoAreYou;
use crate::{
    common::{Error, Result},
    crypto::Crypto,
    err,
    p2p::{
        credential::Credential,
        discovery::whoareyou::{self, WhoAreYouAck},
        peer::{
            self,
            peer_store::{Filter, PeerStore},
            Peer,
        },
    },
};
use k256::ecdsa::{
    signature::{Signer, Verifier},
    Signature, SigningKey,
};
use logger::log;
use std::sync::Arc;
use tokio::{
    io::AsyncWriteExt,
    net::TcpStream,
    sync::{mpsc::Sender, Mutex, MutexGuard},
};

/// E Error
/// I Index (Last accessed peer idx)
pub enum HandleStatus<I, E> {
    NoAvailablePeer,

    IllegalPeerFound(I),

    LocalAddrIdentical,

    ConnectionFail(E),

    WhoAreYouInitiateFail(E),

    WhoAreYouAckReceiveFail(E),

    PeerUpdateFail(E),

    Success(I),
}

pub struct Handler {
    peer_store: Arc<PeerStore>,
    credential: Arc<Credential>,
    peer_op_port: u16,
    my_disc_endpoint: String,
    peer_op_wakeup_tx: Arc<Sender<usize>>,
    last_peer_idx: Arc<Mutex<usize>>,
}

impl Handler {
    pub fn new(
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
        peer_op_port: u16,
        my_disc_endpoint: String,
        peer_op_wakeup_tx: Arc<Sender<usize>>,
        last_peer_idx: Arc<Mutex<usize>>,
    ) -> Handler {
        Handler {
            peer_store,
            credential,
            peer_op_port,
            my_disc_endpoint,
            peer_op_wakeup_tx,
            last_peer_idx,
        }
    }

    pub async fn handle_my_endpoint(
        &self,
        endpoint: String,
        idx: usize,
    ) -> Result<()> {
        log!(
            DEBUG,
            "Discarding dial request, endpoint to local, addr: {}\n",
            endpoint,
        );
        // match self.address_book.remove(idx).await {
        //     Ok(_) => return Ok(()),
        //     Err(err) => return Err(err),
        // }
        Ok(())
    }

    pub async fn a(&self, peer: &mut Peer) {
        println!("{:?}", peer);
    }

    pub async fn run(&mut self) -> HandleStatus<usize, Error> {
        let mut last_peer_idx = self.last_peer_idx.lock().await;

        let peer = self
            .peer_store
            .next(Some(*last_peer_idx), &Filter::not_initialized)
            .await;

        let (mut peer, peer_idx) = match peer {
            Some((peer_guard, idx)) => (peer_guard, idx),
            None => return HandleStatus::NoAvailablePeer
        };

        *last_peer_idx = peer_idx;

        let peer = match &mut *peer {
            Some(a) => a,
            None => return HandleStatus::NoAvailablePeer
        };

        peer.ip = "3".into();

        self.a(peer).await;



        // std::mem::drop(peer);

        // a.peer_id = "3".into();

        // let a = match *peer {
        //     Some(p) => p,
        //     None => return HandleStatus::IllegalPeerFound(0),
        // };

        // let address_book_len = self.address_book.len().await;

        // log!(DEBUG, "Address book len: {}\n", address_book_len);

        // let (addr, idx) =
        // match self.address_book.next(&Filter::not_discovered).await {
        //     Some(a) => a,
        //     None => {
        //         log!(DEBUG, "Cannot acquire next address\n");

        //         return HandleStatus::NoAvailableAddress;
        //     }
        // };

        // let addr = addr.lock().await;
        // let peer = &self.peer;
        // let peer = peer.lock().await;
        // let endpoint = format!("{}:{}", peer.ip, peer.disc_port);

        // println!("44, {}", endpoint);

        // if endpoint == self.my_disc_endpoint {
        //     match self.handle_my_endpoint(endpoint.to_owned(), idx).await {
        //         Ok(_) => return HandleStatus::LocalAddrIdentical,
        //         Err(err) => {
        //             log!(DEBUG, "Error handling my endpoint, err: {}", err);
        //         }
        //     }
        // };

        // let mut stream =
        //     match TcpStream::connect(endpoint.to_owned()).await {
        //         Ok(s) => {
        //             log!(
        //                 DEBUG,
        //                 "Successfully connected to endpoint, {}\n",
        //                 endpoint
        //             );
        //             s
        //         }
        //         Err(err) => {
        //             log!(DEBUG, "Cannot disc dial to endpoint, {}\n", endpoint);

        //             return HandleStatus::ConnectionFail(err.into());
        //         },
        //     };

        // match self.initiate_who_are_you(&mut stream).await {
        //     Ok(_) => (),
        //     Err(err) => return HandleStatus::WhoAreYouInitiateFail(err),
        // };

        // let way_ack = match self.receive_who_are_you_ack(stream).await {
        //     Ok(w) => w,
        //     Err(err) => return HandleStatus::WhoAreYouAckReceiveFail(err),
        // };

        // match self.handle_succeed_who_are_you(way_ack).await {
        //     Ok(_) => (),
        //     Err(err) => return HandleStatus::PeerUpdateFail(err),
        // };

        HandleStatus::Success(0)
    }

    pub async fn initiate_who_are_you(
        &mut self,
        stream: &mut TcpStream,
    ) -> Result<()> {
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
                return err!("Error creating WhoAreYou request, err: {}", err);
            }
        };

        match stream.write_all(&buf).await {
            Ok(_) => Ok(()),
            Err(err) => {
                return err!(
                    "Error sending the whoAreYou buffer, err: {}",
                    err
                );
            }
        }
    }

    pub async fn receive_who_are_you_ack(
        &self,
        mut stream: TcpStream,
    ) -> Result<WhoAreYouAck> {
        let way_ack = match WhoAreYouAck::parse(&mut stream).await {
            Ok(w) => w,
            Err(err) => {
                return err!("Cannot process WhoAreyouAck, err: {}", err);
            }
        };

        let verifying_key = match Crypto::convert_public_key_to_verifying_key(
            way_ack.way.public_key_bytes,
        ) {
            Ok(v) => v,
            Err(err) => {
                return err!("Error creating verifying key, err: {}", err);
            }
        };
        let sig = way_ack.way.sig;

        match verifying_key.verify(whoareyou::MESSAGE, &sig) {
            Ok(_) => (),
            Err(err) => {
                return err!("Signature is invalid, err: {}", err);
            }
        };

        Ok(way_ack)
    }

    pub async fn handle_succeed_who_are_you(
        &mut self,
        way_ack: WhoAreYouAck,
        // mut addr: MutexGuard<'_, Address>,
    ) -> Result<()> {
        // addr.status = Status::DiscoverySuccess;

        // let mut peer = self.peer.lock().await;
        // let peer = &self.peer;
        // let peer = &mut self.peer;
        // peer.status = peer::Status::DiscoverySuccess;
        // peer.peer_id = way_ack.way.peer_id;
        // // peer.ip = addr.ip.to_owned();
        // // peer.disc_port = addr.disc_port;
        // peer.pk_bytes = way_ack.way.public_key_bytes;
        // peer.peer_op_port = way_ack.way.peer_op_port;

        // log!(DEBUG, "Successfully handled disc dial peer: {:?}\n", peer);

        // let peer_op_wakeup_tx = self.peer_op_wakeup_tx.clone();

        // let wakeup = tokio::spawn(async move {
        //     match peer_op_wakeup_tx.send(0).await {
        //         Ok(_) => Ok(()),
        //         Err(err) => {
        //             return err!(
        //                 "Error sending peer op wakeup msg, err: {}",
        //                 err
        //             );
        //         }
        //     }
        // });

        // match wakeup.await {
        //     Ok(_) => {
        //         Ok(())
        //     },
        //     Err(err) => return Err(err.into()),
        // }
        Ok(())
    }
}
