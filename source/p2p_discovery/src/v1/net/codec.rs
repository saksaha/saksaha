use std::error::Error;

use bytes::BytesMut;
use futures::{SinkExt, StreamExt};
use tokio_util::codec::{
    AnyDelimiterCodec, BytesCodec, Decoder, Encoder, LinesCodec,
};
use tokio_util::udp::UdpFramed;

use crate::msg::Msg;

pub(crate) struct UdpCodec {}

impl UdpCodec {
    pub fn a() {
        let t = tokio::spawn(async move {
            let (socket1, socket_addr1) =
                utils_net::setup_udp_socket(Some(0)).await.unwrap();

            let (socket2, socket_addr2) =
                utils_net::setup_udp_socket(Some(0)).await.unwrap();

            let mut udp_framed1 = UdpFramed::new(socket1, LinesCodec::new());

            let mut udp_framed2 = UdpFramed::new(socket2, LinesCodec::new());
            udp_framed2.send(("foo\nbar", socket_addr1)).await.unwrap();

            loop {
                match udp_framed1.next().await {
                    Some(m) => {
                        let (msg, addr) = m.unwrap();
                        println!("msg arrived, msg: {}, addr: {}", msg, addr);
                    }
                    None => {}
                };
            }
        });
    }
}

impl Encoder<Msg> for UdpCodec {
    type Error = Box<dyn Error>;

    fn encode(
        &mut self,
        msg: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
