use super::UdpCodec;
use crate::Msg;
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use std::net::SocketAddr;
use tokio::{net::UdpSocket, sync::RwLock};
use tokio_util::udp::UdpFramed;

pub(crate) struct Connection {
    pub(crate) tx: RwLock<SplitSink<UdpFramed<UdpCodec>, (Msg, SocketAddr)>>,
    pub(crate) rx: RwLock<SplitStream<UdpFramed<UdpCodec>>>,
}

impl Connection {
    pub(crate) fn new(socket: UdpSocket) -> Connection {
        let udp_codec = UdpCodec {};

        let (tx, rx) = {
            let f = UdpFramed::new(socket, udp_codec);
            let (tx, rx) = f.split();

            (RwLock::new(tx), RwLock::new(rx))
        };

        Connection { tx, rx }
    }
}
