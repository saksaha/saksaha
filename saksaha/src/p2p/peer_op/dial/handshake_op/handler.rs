use crate::{
    common::{Error, Result},
    p2p::{peer::Peer, peer_op},
};
use logger::log;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::Mutex};

pub enum HandleStatus<E> {
    ConnectionFail(E),

    HandshakeInitiateFail(E),

    Success,
}

pub struct Handler {
    peer: Arc<Mutex<Peer>>,
    // credential: Arc<Credential>,
    // peer_op_port: u16,
    // address_book: Arc<AddressBook>,
    // my_disc_endpoint: String,
    // peer_op_wakeup_tx: Arc<Sender<usize>>,
}

impl Handler {
    pub fn new(
        peer: Arc<Mutex<Peer>>,
        // credential: Arc<Credential>,
        // peer_op_port: u16,
        // address_book: Arc<AddressBook>,
    ) -> Handler {
        Handler { peer }
    }

    pub async fn run(&self) -> HandleStatus<Error> {
        let peer = self.peer.lock().await;
        let peer_op_endpoint = format!("{}:{}", peer.ip, peer.peer_op_port);

        let mut stream =
            match TcpStream::connect(peer_op_endpoint.to_owned()).await {
                Ok(s) => {
                    log!(
                        DEBUG,
                        "Successfully connected to endpoint, {}\n",
                        peer_op_endpoint.to_owned(),
                    );
                    s
                }
                Err(err) => return HandleStatus::ConnectionFail(err.into()),
            };

        match self.initiate_handshake(&mut stream).await {
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
    ) -> Result<()> {
        Ok(())
    }

    pub async fn receive_handshake_ack(&self, stream: TcpStream) -> Result<()> {

        Ok(())
    }
}
