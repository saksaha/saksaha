use crate::{
    common::{Error, Result},
    err,
    p2p::{credential::Credential, peer::Peer},
};
use k256::{ecdh::EphemeralSecret, EncodedPoint, PublicKey, SecretKey};
use logger::log;
use std::sync::Arc;
use tokio::{
    net::TcpStream,
    sync::{Mutex, MutexGuard},
};

pub enum HandleStatus<E> {
    ConnectionFail(E),

    HandshakeInitiateFail(E),

    Success,
}

pub struct Handler<'a> {
    peer: MutexGuard<'a, Peer>,
    credential: Arc<Credential>,
    // peer_op_port: u16,
    // address_book: Arc<AddressBook>,
    // my_disc_endpoint: String,
    // peer_op_wakeup_tx: Arc<Sender<usize>>,
}

impl<'a> Handler<'a> {
    pub fn new(
        // peer: Arc<Mutex<Peer>>,
        peer: MutexGuard<Peer>,
        credential: Arc<Credential>,
        // peer_op_port: u16,
        // address_book: Arc<AddressBook>,
    ) -> Handler {
        Handler { peer, credential }
    }

    pub async fn run(&self) -> HandleStatus<Error> {
        // let peer = self.peer.lock().await;
        let peer = &self.peer;
        let peer_op_endpoint = format!("{}:{}", peer.ip, peer.peer_op_port);
        let credential = self.credential.clone();

        let mut stream =
            match TcpStream::connect(peer_op_endpoint.to_owned()).await {
                Ok(s) => {
                    log!(
                        DEBUG,
                        "Successfully connected to endpoint, {}",
                        peer_op_endpoint.to_owned(),
                    );
                    s
                }
                Err(err) => return HandleStatus::ConnectionFail(err.into()),
            };

        match self.initiate_handshake(&mut stream, peer, credential).await {
            Ok(_) => (),
            Err(err) => return HandleStatus::HandshakeInitiateFail(err),
        };

        match self.receive_handshake_ack(stream).await {
            Ok(_) => (),
            Err(err) => return HandleStatus::HandshakeInitiateFail(err),
        };

        HandleStatus::Success
    }

    pub async fn initiate_handshake(
        &self,
        stream: &mut TcpStream,
        peer: &MutexGuard<'_, Peer>,
        credential: Arc<Credential>,
    ) -> Result<()> {
        let peer_pk_bytes = peer.public_key_bytes;
        // PublicKey::f

        println!("33, {:?}", peer_pk_bytes);

        let a = match EncodedPoint::from_bytes(&peer_pk_bytes) {
            Ok(a) => a,
            Err(err) => {
                println!("aa: {}", err);
                return err!("f");
            }
        };
        // let peer_pk = match PublicKey::from_sec1_bytes(&peer_pk_bytes) {
        //     Ok(p) => p,
        //     Err(err) => {
        //         println!("33, err: {}", err);
        //         return err!("ff");
        //     }
        // };
        println!("1 {:?},", a);

        // let a = SecretKey::from();
        let a = credential.secret_key.to_bytes();
        let b = SecretKey::from_bytes(a);
        println!("2, {:?}", b);

        // let secret_key = credential.secret_key;
        // EphemeralSecret::from(peer.peer_id);
        // let peer_pub_key = EncodedPoint::from(peer.peer_id);
        // let secret =

        // let handshake = Handshake::new();

        Ok(())
    }

    pub async fn receive_handshake_ack(&self, stream: TcpStream) -> Result<()> {
        Ok(())
    }
}
