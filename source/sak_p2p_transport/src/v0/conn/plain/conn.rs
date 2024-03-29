use super::codec::P2PCodec;
use crate::{TrptError, UpgradedConn, UpgradedP2PCodec};
use chacha20::{cipher::KeyIvInit, ChaCha20};
use sak_crypto::{
    sha3::{Digest, Keccak256},
    SharedSecret,
};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub struct Conn {
    pub socket_addr: SocketAddr,
    pub socket: Framed<TcpStream, P2PCodec>,
    public_key: String,
}

impl Conn {
    pub fn new(socket: TcpStream, my_public_key: String) -> Result<Conn, TrptError> {
        let socket_addr = socket.peer_addr()?;

        let p2p_codec = P2PCodec {};

        let socket = Framed::new(socket, p2p_codec);

        let c = Conn {
            socket_addr,
            socket,
            public_key: my_public_key,
        };

        Ok(c)
    }

    pub async fn upgrade(
        self,
        shared_secret: SharedSecret,
        nonce: &[u8],
        her_public_key: &String,
    ) -> Result<UpgradedConn, TrptError> {
        // Initialize message authentication code (MAC)
        let (out_mac, in_mac) = {
            let mut out_mac = Keccak256::default();
            out_mac.update(shared_secret.as_bytes());

            let mut in_mac = Keccak256::default();
            in_mac.update(shared_secret.as_bytes());

            (out_mac, in_mac)
        };

        let out_cipher = ChaCha20::new(shared_secret.as_bytes().into(), nonce.into());

        let in_cipher = ChaCha20::new(shared_secret.as_bytes().into(), nonce.into());

        let conn_id = format!(
            "{}-{}",
            "me",
            sak_p2p_id::make_public_key_short(&her_public_key)?
        );

        let socket = self.socket.map_codec(|_| UpgradedP2PCodec {
            out_cipher,
            in_cipher,
            out_mac,
            in_mac,
            conn_id: conn_id.to_string(),
            parsed_msg_len: None,
            in_count: 0,
            out_count: 0,
        });

        let upgraded_conn = UpgradedConn::init(socket, conn_id, self.public_key).await;

        Ok(upgraded_conn)
    }
}
