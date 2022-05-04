// use crate::{Connection, Frame, TransportMeta};
// use bytes::Bytes;
// use log::{debug, error};
// use p2p_identity::PeerId;
// use std::sync::Arc;
// use thiserror::Error;
// use tokio::net::TcpStream;
use p2p_identity::addr::Addr;

// #[derive(Error, Debug)]
// pub enum TransportInitError {
//     #[error("Request to my (recursive) endpoint: {endpoint}")]
//     MyEndpoint { endpoint: String },

//     #[error("Can't connect to endpoint")]
//     ConnectionFail {
//         #[from]
//         source: std::io::Error,
//     },

//     #[error("Already talking: {ip}")]
//     CallInProcess { ip: String },
// }

pub async fn initiate_handshake(
    // ip: String,
    // p2p_port: u16,
    // transport_meta: Arc<TransportMeta>,
    addr: &Addr,
) -> Result<(), String> {
    // let my_p2p_port = transport_meta.my_p2p_port;
    // let active_calls = transport_meta.active_calls.clone();

    // let endpoint = format!("{}:{}", ip, p2p_port);

    // let endpoint = addr.

    // if active_calls.contain(&ip).await {
    //     return Err(TransportInitError::CallInProcess { ip });
    // }

    // if super::is_my_endpoint(my_p2p_port, &endpoint) {
    //     return Err(TransportInitError::MyEndpoint { endpoint });
    // }

    // let mut conn = match TcpStream::connect(endpoint).await {
    //     Ok(s) => {
    //         println!("called, ip: {}", ip);

    //         Connection::new(s)
    //     }
    //     Err(err) => {
    //         return Err(TransportInitError::ConnectionFail { source: err })
    //     }
    // };

    // let mut frame = Frame::array();
    // frame.push_bulk(Bytes::from("power".as_bytes()));
    // match conn.write_frame(&frame).await {
    //     Ok(_) => (),
    //     Err(err) => {
    //         println!("err: {}", err);
    //     }
    // }

    // // conn.write_frame()

    // // let my_sig = self.disc_state.id.sig();
    // // let my_public_key_bytes = self.disc_state.id.public_key_bytes();

    // // let way_syn =
    // //     WhoAreYouSyn::new(my_sig, my_p2p_port, my_public_key_bytes);

    // // let buf = match way_syn.to_bytes() {
    // //     Ok(b) => b,
    // //     Err(err) => {
    // //         return Err(WhoareyouInitError::ByteConversionFail { err });
    // //     }
    // // };

    // // self.udp_socket.send_to(&buf, endpoint.clone()).await?;

    // // debug!(
    // //     "Successfully sent WhoAreYou to endpoint: {}, buf len: {}",
    // //     &endpoint,
    // //     buf.len()
    // // );

    Ok(())
}
