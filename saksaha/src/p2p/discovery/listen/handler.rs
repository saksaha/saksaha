use crate::{
    common::{Error, Result},
    err,
    p2p::{
        address::{address_book::AddressBook, Address, Status as AddrStatus},
        credential::Credential,
        discovery::whoareyou::{self, WhoAreYou, WhoAreYouAck},
        peer::{Peer, Status as PeerStatus},
    },
};
use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use logger::log;
use std::sync::Arc;
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};

pub enum HandleStatus<E> {
    WhoAreYouReceiveFail(E),

    WhoAreYouAckInitiateFail(E),

    PeerUpdateFail(E),

    JoinError(E),

    Success,
}

pub struct Handler {
    addr: Arc<Mutex<Address>>,
    address_book: Arc<AddressBook>,
    stream: TcpStream,
    peer: Arc<Mutex<Peer>>,
    credential: Arc<Credential>,
    peer_op_port: u16,
}

impl Handler {
    pub fn new(
        addr: Arc<Mutex<Address>>,
        address_book: Arc<AddressBook>,
        stream: TcpStream,
        peer: Arc<Mutex<Peer>>,
        credential: Arc<Credential>,
        peer_op_port: u16,
    ) -> Handler {
        Handler {
            addr,
            address_book,
            stream,
            peer,
            credential,
            peer_op_port,
        }
    }

    pub async fn run(&mut self) -> HandleStatus<Error> {
        let way = match self.receive_who_are_you().await {
            Ok(w) => w,
            Err(err) => return HandleStatus::WhoAreYouReceiveFail(err),
        };

        match self.initate_who_are_you_ack().await {
            Ok(_) => (),
            Err(err) => return HandleStatus::WhoAreYouAckInitiateFail(err),
        };

        match self.handle_succeed_who_are_you(way).await {
            Ok(_) => (),
            Err(err) => return HandleStatus::PeerUpdateFail(err),
        };

        HandleStatus::Success
    }

    pub async fn receive_who_are_you(&mut self) -> Result<WhoAreYou> {
        match WhoAreYou::parse(&mut self.stream).await {
            Ok(w) => Ok(w),
            Err(err) => {
                return err!("Error parsing who are you request, err: {}", err);
            }
        }
    }

    pub async fn initate_who_are_you_ack(&mut self) -> Result<()> {
        let secret_key = &self.credential.secret_key;
        let signing_key = SigningKey::from(secret_key);
        let sig: Signature = signing_key.sign(whoareyou::MESSAGE);

        let way_ack = WhoAreYouAck::new(
            sig,
            self.peer_op_port,
            self.credential.public_key_bytes,
        );

        let buf = match way_ack.to_bytes() {
            Ok(b) => b,
            Err(err) => {
                return err!(
                    "Error converting WhoAreYouAck to bytes, err: {}",
                    err
                );
            }
        };

        match &self.stream.write_all(&buf).await {
            Ok(_) => (),
            Err(err) => {
                return err!(
                    "Error sending the whoAreYou buffer, err: {}",
                    err
                );
            }
        }

        Ok(())
    }

    pub async fn handle_succeed_who_are_you(
        &self,
        way: WhoAreYou,
    ) -> Result<()> {
        let peer_addr = match self.stream.peer_addr() {
            Ok(a) => a,
            Err(err) => return Err(err.into()),
        };

        let addr = self.addr.clone();
        let mut addr = addr.lock().await;
        // addr.ip =
        addr.status = AddrStatus::DiscoverySuccess;

        let mut peer = self.peer.lock().await;
        peer.status = PeerStatus::DiscoverySuccess;
        peer.ip = peer_addr.ip().to_string();
        peer.disc_port = peer_addr.port();
        peer.peer_op_port = way.peer_op_port;
        peer.peer_id = way.peer_id;

        log!(DEBUG, "Successfully handled disc listen, peer: {:?}\n", peer);

        Ok(())
    }
}
