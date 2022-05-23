use crate::msg::Msg2;

use super::codec::UdpCodec;
use futures::{
    stream::{SplitSink, SplitStream},
    StreamExt,
};

use futures::sink::SinkExt;
use std::{net::SocketAddr, sync::Arc};
use tokio::{net::UdpSocket, sync::RwLock};
use tokio_util::udp::UdpFramed;

pub(crate) struct Connection2 {
    // pub(crate) socket: Arc<RwLock<UdpFramed<UdpCodec>>>,
    pub(crate) tx: RwLock<SplitSink<UdpFramed<UdpCodec>, (Msg2, SocketAddr)>>,
    pub(crate) rx: RwLock<SplitStream<UdpFramed<UdpCodec>>>,
}

impl Connection2 {
    pub fn new(socket: UdpSocket) -> Connection2 {
        let udp_codec = UdpCodec {};

        let (tx, rx) = {
            let f = UdpFramed::new(socket, udp_codec);
            let (tx, rx) = f.split();

            (RwLock::new(tx), RwLock::new(rx))
        };

        // f.next();
        // let (rx, tx) = f.split();

        // let udp_codec = UdpCodec {};
        // let mut udp_framed2 = UdpFramed::new(socket2, udp_codec);
        // udp_framed2.send(("foo\nbar", socket_addr1)).await.unwrap();

        // loop {
        //     match udp_framed1.next().await {
        //         Some(m) => {
        //             let (msg, addr) = m.unwrap();
        //             // println!("msg arrived, msg: {}, addr: {}", msg, addr);
        //         }
        //         None => {}
        //     };
        // }

        Connection2 {
            // socket: framed_socket,
            tx,
            rx,
        }
    }
}
