use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use std::sync::Arc;
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::OwnedMutexGuard};

pub enum HandleError {
    NoAvailablePeerSlot,

    AddressAcquireFail(String),

    PeerAlreadyTalking(String),

    WhoAreYouReceiveFail(String),

    WhoAreYouAckInitiateFail(String),

    PeerUpdateFail(String),

    Success,
}

pub struct Handler {
    stream: TcpStream,
    // peer_store: Arc<PeerStore>,
    // credential: Arc<Credential>,
    peer_op_port: u16,
}

impl Handler {
    pub fn new(
        stream: TcpStream,
        // peer_store: Arc<PeerStore>,
        // credential: Arc<Credential>,
        peer_op_port: u16,
    ) -> Handler {
        Handler {
            stream,
            // peer_store,
            // credential,
            peer_op_port,
        }
    }

    pub async fn run(&mut self) -> Result<(), HandleError> {
        // let peer_store = self.peer_store.clone();
        // let (peer_ip, peer_port) = match self.stream.peer_addr() {
        //     Ok(a) => (a.ip().to_string(), a.port()),
        //     Err(err) => return HandleStatus::AddressAcquireFail(err.into()),
        // };

        // let peer_found = peer_store
        //     .find(&|peer| {
        //         if peer.ip == peer_ip && peer.disc_port == peer_port {
        //             return true;
        //         }
        //         return false;
        //     })
        //     .await;

        // match peer_found {
        //     Some(_) => {
        //         let endpoint = format!("{}:{}", peer_ip, peer_port);

        //         return HandleStatus::PeerAlreadyTalking(endpoint);
        //     }
        //     None => (),
        // }

        // let (mut peer, _) = match peer_store.reserve().await {
        //     Some(p) => p,
        //     None => return HandleStatus::NoAvailablePeerSlot,
        // };

        // let way = match self.receive_who_are_you().await {
        //     Ok(w) => w,
        //     Err(err) => {
        //         peer.record_fail();

        //         return HandleStatus::WhoAreYouReceiveFail(err);
        //     }
        // };

        // match self.initate_who_are_you_ack().await {
        //     Ok(_) => (),
        //     Err(err) => {
        //         peer.record_fail();

        //         return HandleStatus::WhoAreYouAckInitiateFail(err);
        //     }
        // };

        // match self
        //     .handle_succeed_who_are_you(way, peer, peer_ip, peer_port)
        //     .await
        // {
        //     Ok(_) => (),
        //     Err(err) => {
        //         return HandleStatus::PeerUpdateFail(err);
        //     }
        // };

        // HandleStatus::Success
        Ok(())
    }

    // pub async fn receive_who_are_you(&mut self) -> Result<WhoAreYouMsg> {
    //     let way = match WhoAreYouMsg::parse(&mut self.stream).await {
    //         Ok(w) => {
    //             // log!(DEBUG, "Received WhoAreYou, raw: {:?}", w.raw);

    //             w
    //         }
    //         Err(err) => {
    //             return err!("Error parsing who are you request, err: {}", err);
    //         }
    //     };

    //     Ok(way)
    // }

    // pub async fn initate_who_are_you_ack(&mut self) -> Result<()> {
    //     let secret_key = &self.credential.secret_key;
    //     let signing_key = SigningKey::from(secret_key);
    //     let sig: Signature = signing_key.sign(msg::codec::MESSAGE);

    //     let way_ack = WhoAreYouAck::new(
    //         sig,
    //         self.peer_op_port,
    //         self.credential.public_key_bytes,
    //     );

    //     let buf = match way_ack.to_bytes() {
    //         Ok(b) => b,
    //         Err(err) => {
    //             return err!(
    //                 "Error converting WhoAreYouAck to bytes, err: {}",
    //                 err
    //             );
    //         }
    //     };

    //     match &self.stream.write_all(&buf).await {
    //         Ok(_) => (),
    //         Err(err) => {
    //             return err!(
    //                 "Error sending the whoAreYou buffer, err: {}",
    //                 err
    //             );
    //         }
    //     }

    //     Ok(())
    // }

    // pub async fn handle_succeed_who_are_you(
    //     &mut self,
    //     way: WhoAreYouMsg,
    //     mut peer: OwnedMutexGuard<Peer>,
    //     peer_ip: String,
    //     peer_port: u16,
    // ) -> Result<()> {
    //     peer.ip = peer_ip;
    //     peer.disc_port = peer_port;
    //     peer.peer_id = way.peer_id;
    //     peer.peer_op_port = way.peer_op_port;
    //     peer.public_key_bytes = way.public_key_bytes;
    //     peer.fail_count = 0;
    //     peer.status = peer::Status::DiscoverySuccess;

    //     log!(
    //         DEBUG,
    //         "[PeerDiscovered], disc - listen, peer: {}",
    //         peer.short_url(),
    //     );

    //     Ok(())
    // }
}
