use super::whoareyou::WhoAreYou;
use crate::{
    common::{Error, Result},
    crypto::Crypto,
    err,
    p2p::{
        address::{status::Status, Address, AddressBook},
        credential::Credential,
        discovery::whoareyou::{self, WhoAreYouAck},
        peer::{Peer, PeerStatus},
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
    sync::{Mutex, MutexGuard},
};

pub enum HandleStatus<E> {
    AddressNotFound,

    LocalAddrIdentical,

    ConnectionFail(E),

    WhoAreYouInitiateFail(E),

    WhoAreYouAckReceiveFail(E),

    PeerUpdateFail(E),

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
        match self.address_book.remove(idx).await {
            Ok(_) => return Ok(()),
            Err(err) => return Err(err),
        }
    }

    pub async fn run(&mut self) -> HandleStatus<Error> {
        let address_book_len = self.address_book.len().await;

        log!(DEBUG, "Address book len: {}\n", address_book_len);

        let (addr, idx) = match self
            .address_book
            .next(Some(&get_not_initialized_addr))
            .await
        {
            Some(a) => a,
            None => {
                log!(DEBUG, "Cannot acquire next address\n");

                return HandleStatus::AddressNotFound;
            }
        };

        let addr = addr.lock().await;

        if addr.endpoint == self.my_disc_endpoint {
            match self.handle_my_endpoint(addr.endpoint.to_owned(), idx).await {
                Ok(_) => return HandleStatus::LocalAddrIdentical,
                Err(err) => {
                    log!(DEBUG, "Error handling my endpoint, err: {}", err);
                }
            }
        };

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
                Err(err) => return HandleStatus::ConnectionFail(err.into()),
            };

        match self.initiate_who_are_you(&mut stream).await {
            Ok(_) => (),
            Err(err) => return HandleStatus::WhoAreYouInitiateFail(err),
        };

        let way_ack = match self.receive_who_are_you_ack(stream).await {
            Ok(w) => w,
            Err(err) => return HandleStatus::WhoAreYouAckReceiveFail(err),
        };

        match self.handle_succeed_who_are_you(way_ack, addr).await {
            Ok(_) => (),
            Err(err) => return HandleStatus::PeerUpdateFail(err),
        };

        HandleStatus::Success
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
        &self,
        way_ack: WhoAreYouAck,
        mut addr: MutexGuard<'_, Address>,
    ) -> Result<()> {
        let mut peer = self.peer.lock().await;
        peer.status = PeerStatus::Discovered;
        peer.endpoint = addr.endpoint.to_owned();
        peer.peer_id = way_ack.way.get_peer_id();
        addr.status = Status::HandshakeSucceeded;

        log!(DEBUG, "Successfully discovered a peer: {:?}", peer);

        tokio::spawn(async move {
            println!("Start synchroize");
        });

        Ok(())
    }
}

fn get_not_initialized_addr(addr: MutexGuard<Address>) -> bool {
    addr.status == Status::NotInitialized
}
