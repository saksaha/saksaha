use std::sync::Arc;

use super::codec::UdpCodec;
use tokio::{net::UdpSocket, sync::RwLock};
use tokio_util::udp::UdpFramed;

pub(crate) struct Connection2 {
    pub(crate) socket: Arc<RwLock<UdpFramed<UdpCodec>>>,
}

impl Connection2 {
    pub fn new(socket: UdpSocket) -> Connection2 {
        let udp_codec = UdpCodec {};

        let mut framed_socket = {
            let f = UdpFramed::new(socket, udp_codec);

            Arc::new(RwLock::new(f))
        };

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
            socket: framed_socket,
        }
    }
}
