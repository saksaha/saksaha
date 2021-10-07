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

    IllegalEndpoint(E),

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
    disc_port: u16,
    peer_op_wakeup_tx: Arc<Sender<usize>>,
    last_peer_idx: Arc<Mutex<usize>>,
}

impl Handler {
    pub fn new(
        peer_store: Arc<PeerStore>,
        credential: Arc<Credential>,
        peer_op_port: u16,
        disc_port: u16,
        peer_op_wakeup_tx: Arc<Sender<usize>>,
        last_peer_idx: Arc<Mutex<usize>>,
    ) -> Handler {
        Handler {
            peer_store,
            credential,
            peer_op_port,
            disc_port,
            peer_op_wakeup_tx,
            last_peer_idx,
        }
    }

    pub fn require_not_my_endpoint(
        &self,
        peer: &mut MutexGuard<Peer>,
    ) -> Result<String> {
        let endpoint = format!("{}:{}", peer.ip, peer.disc_port);
        let my_disc_endpoint = format!("127.0.0.1:{}", self.disc_port);

        println!(
            "{} {} {}",
            endpoint,
            my_disc_endpoint,
            endpoint == my_disc_endpoint
        );

        if endpoint == my_disc_endpoint {
            log!(
                DEBUG,
                "Discarding dial request, endpoint to local, addr: {}\n",
                endpoint,
            );

            peer.empty();
            return err!(
                "Endpoint identical, removing this peer, peer endpoint: {}",
                endpoint
            );
        }

        Ok(endpoint)
    }

    pub async fn run(&self) -> HandleStatus<usize, Error> {
        let mut last_peer_idx = self.last_peer_idx.lock().await;
        let credential = self.credential.clone();

        let peer_store = self.peer_store.clone();

        let peer =
            peer_store.next(Some(*last_peer_idx), &Filter::not_initialized);

        let (mut peer, peer_idx) = match peer {
            Some((p, idx)) => (p, idx),
            None => return HandleStatus::NoAvailablePeer,
        };

        *last_peer_idx = peer_idx;

        let endpoint = match self.require_not_my_endpoint(&mut peer) {
            Ok(ep) => ep,
            Err(err) => return HandleStatus::IllegalEndpoint(err),
        };

        let mut stream = match TcpStream::connect(endpoint.to_owned()).await {
            Ok(s) => {
                log!(
                    DEBUG,
                    "Successfully connected to endpoint, {}\n",
                    endpoint
                );
                s
            }
            Err(err) => {
                let msg = format!(
                    "Cannot connect to peer.ip: {}, port: {}, err: {}",
                    peer.ip, peer.disc_port, err
                );
                let err = Error::new_default(msg);

                peer.record_fail();

                return HandleStatus::ConnectionFail(err);
            }
        };

        match self.initiate_who_are_you(&mut stream).await {
            Ok(_) => (),
            Err(err) => {
                peer.record_fail();

                return HandleStatus::WhoAreYouInitiateFail(err);
            }
        };

        let way_ack = match self.receive_who_are_you_ack(stream).await {
            Ok(w) => w,
            Err(err) => {
                peer.record_fail();

                return HandleStatus::WhoAreYouAckReceiveFail(err);
            }
        };

        match self.handle_succeed_who_are_you(way_ack, peer).await {
            Ok(_) => (),
            Err(err) => return HandleStatus::PeerUpdateFail(err),
        };

        HandleStatus::Success(0)
    }

    pub async fn initiate_who_are_you(
        &self,
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
        &self,
        way_ack: WhoAreYouAck,
        mut peer: MutexGuard<'_, Peer>,
    ) -> Result<()> {
        let peer_op_wakeup_tx = self.peer_op_wakeup_tx.clone();

        peer.peer_id = way_ack.way.peer_id;
        peer.peer_op_port = way_ack.way.peer_op_port;
        peer.public_key_bytes = way_ack.way.public_key_bytes;
        peer.status = peer::Status::DiscoverySuccess;
        peer.fail_count = 0;

        let wakeup = tokio::spawn(async move {
            match peer_op_wakeup_tx.send(0).await {
                Ok(_) => Ok(()),
                Err(err) => {
                    return err!(
                        "Error sending peer op wakeup msg, err: {}",
                        err
                    );
                }
            }
        });

        match wakeup.await {
            Ok(_) => (),
            Err(err) => return Err(err.into()),
        }

        log!(DEBUG, "Successfully handled disc dial peer: {:?}\n", peer);

        Ok(())
    }
}

fn compare_public_key(src: [u8; 65], tar: [u8; 65]) -> Result<bool> {
    for i in 0..src.len() {
        return Ok(src[i] > tar[i]);
    }
    err!("Two public keys are the same")
}
