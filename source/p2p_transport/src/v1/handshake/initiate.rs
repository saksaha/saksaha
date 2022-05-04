// use crate::{Connection, Frame, TransportMeta};
// use bytes::Bytes;
// use log::{debug, error};
// use p2p_identity::PeerId;
// use std::sync::Arc;
use crate::connection::Connection;
use thiserror::Error;
use tokio::net::TcpStream;

// use tokio::net::TcpStream;
use super::check;
use p2p_identity::addr::Addr;

pub struct HandshakeInitArgs {
    pub p2p_port: u16,
    pub addr: Addr,
}

#[derive(Error, Debug)]
pub enum HandshakeInitError {
    #[error("P2P Port may not be provided")]
    InvalidP2PEndpoint,

    #[error("Cannot send request to myself, addr: {addr}")]
    MyEndpoint { addr: Addr },

    #[error("Cannot craete tcp stream into endpoint")]
    ConnectionFail { err: String },
    // #[error("Already talking: {ip}")]
    // CallInProcess { ip: String },
}

pub async fn initiate_handshake(
    handshake_init_args: HandshakeInitArgs,
) -> Result<(), HandshakeInitError> {
    // let my_p2p_port = transport_meta.my_p2p_port;
    // let active_calls = transport_meta.active_calls.clone();

    let HandshakeInitArgs { p2p_port, addr, .. } = handshake_init_args;

    let endpoint = match addr.p2p_endpoint() {
        Some(e) => e,
        None => return Err(HandshakeInitError::InvalidP2PEndpoint),
    };

    if check::is_my_endpoint(p2p_port, &addr) {
        return Err(HandshakeInitError::MyEndpoint { addr });
    }

    let mut conn = match TcpStream::connect(endpoint).await {
        Ok(s) => {
            println!("called, ip: {:?}", s.peer_addr());

            Connection::new(s)
        }
        Err(err) => {
            return Err(HandshakeInitError::ConnectionFail {
                err: err.to_string(),
            })
        }
    };

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
