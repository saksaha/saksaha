use crate::{
    common::SakResult,
    err_res,
    p2p::{
        credential::Credential,
        discovery::whoareyou::{self, WhoAreYou, WhoAreYouAck},
        peer::Peer,
    },
};
use k256::ecdsa::{signature::Signer, Signature, SigningKey};
use std::sync::Arc;
use tokio::{io::AsyncWriteExt, net::TcpStream, sync::Mutex};

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
            stream,
            peer,
            credential,
            peer_op_port,
        }
    }

    pub async fn run(&mut self) -> SakResult<bool> {
        let way = match WhoAreYou::parse(&mut self.stream).await {
            Ok(w) => w,
            Err(err) => {
                return err_res!(
                    "Error parsing who are you request, err: {}",
                    err
                );
            }
        };

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
                return err_res!(
                    "Error converting WhoAreYouAck to bytes, err: {}",
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

        println!("listen received way: {:?}\n", way.to_bytes());

        Ok(true)
    }
}
