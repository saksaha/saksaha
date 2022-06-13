use super::codec::P2PCodec;
use crate::{BoxedError, Msg};
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio_util::codec::Framed;

pub struct Connection {
    pub socket_addr: SocketAddr,
    // pub socket_tx: SplitSink<Framed<TcpStream, P2PCodec>, Msg>,
    // pub socket_rx: SplitStream<Framed<TcpStream, P2PCodec>>,
    pub socket: Framed<TcpStream, P2PCodec>,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Result<Connection, BoxedError> {
        let socket_addr = socket.peer_addr()?;

        let p2p_codec = P2PCodec {};

        let socket = {
            let f = Framed::new(socket, p2p_codec);

            f
        };

        let c = Connection {
            socket_addr,
            // socket_tx: tx,
            // socket_rx: rx,
            socket,
        };

        Ok(c)
    }
}
