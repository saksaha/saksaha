use super::codec::P2PCodec;
use crate::{BoxedError, Msg};
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use p2p_frame::Frame;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::RwLock;
use tokio_util::codec::Framed;

pub struct Connection {
    pub socket_addr: SocketAddr,
    pub socket_tx: SplitSink<Framed<TcpStream, P2PCodec>, Msg>,
    pub socket_rx: SplitStream<Framed<TcpStream, P2PCodec>>,
}

impl Connection {
    pub fn new(socket: TcpStream) -> Result<Connection, BoxedError> {
        let socket_addr = socket.peer_addr()?;

        let p2p_codec = P2PCodec {};

        let (tx, rx) = {
            let f = Framed::new(socket, p2p_codec);
            let (tx, rx) = f.split();

            // (Arc::new(RwLock::new(tx)), Arc::new(RwLock::new(rx)))
            (tx, rx)
        };

        let c = Connection {
            socket_addr,
            socket_tx: tx,
            socket_rx: rx,
        };

        Ok(c)
    }
}
