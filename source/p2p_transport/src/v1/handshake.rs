use crate::{Connection, Frame};
use bytes::Bytes;
use log::{debug, error};
use p2p_identity::{Identity, PeerId};
use peer::Peer;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::TcpStream;

#[derive(Error, Debug)]
pub enum TransportInitError {
    #[error("Request to my (recursive) endpoint: {endpoint}")]
    MyEndpoint { endpoint: String },

    #[error("Can't connect to endpoint")]
    ConnectionFail {
        #[from]
        source: std::io::Error,
    },

    #[error("Already talking: {ip}")]
    CallInProcess { ip: String },
}

// pub(crate) struct TransportMeta {
//     pub identity: Arc<Identity>,
//     pub my_rpc_port: u16,
//     pub my_p2p_port: u16,
//     pub active_calls: Arc<ActiveCalls>,
// }

// pub struct TransportFactory {
//     transport_meta: Arc<TransportMeta>,
// }

// impl TransportFactory {
//     pub fn new(
//         identity: Arc<Identity>,
//         my_rpc_port: u16,
//         my_p2p_port: u16,
//     ) -> TransportFactory {
//         let active_calls = {
//             let c = ActiveCalls::new();
//             Arc::new(c)
//         };

//         let transport_meta = {
//             let m = TransportMeta {
//                 identity,
//                 my_rpc_port,
//                 my_p2p_port,
//                 active_calls,
//             };
//             Arc::new(m)
//         };

//         TransportFactory { transport_meta }
//     }

//     // pub async fn initiate_handshake(
//     //     &self,
//     //     ip: String,
//     //     p2p_port: u16,
//     //     peer_id: PeerId,
//     //     peer: Arc<Peer>,
//     // ) -> Result<(), TransportInitError> {
//     //     let transport_meta = self.transport_meta.clone();

//     //     let handshake_sent =
//     //         initiate::send_handshake_syn(ip, p2p_port, transport_meta).await?;

//     //     Ok(())
//     // }
// }

#[derive(Clone)]
pub struct HandshakeArgs {
    pub identity: Arc<Identity>,
    pub my_rpc_port: u16,
    pub my_p2p_port: u16,
    pub her_ip: String,
    pub her_p2p_port: u16,
    pub her_public_key: PeerId,
    pub peer: Arc<Peer>,
}

pub async fn initiate_handshake(
    handshake_args: HandshakeArgs,
) -> Result<(), TransportInitError> {
    let my_rpc_port = handshake_args.my_rpc_port;
    let my_p2p_port = handshake_args.my_p2p_port;
    let her_ip = handshake_args.her_ip;
    let her_p2p_port = handshake_args.her_p2p_port;
    let her_peer_id = handshake_args.her_public_key;

    let endpoint = format!("{}:{}", her_ip.clone(), her_p2p_port);

    if super::is_my_endpoint(handshake_args.my_p2p_port, &endpoint) {
        return Err(TransportInitError::MyEndpoint { endpoint });
    }

    let mut conn = match TcpStream::connect(&endpoint).await {
        Ok(s) => {
            debug!(
                "Transport handshake: Successfully connected to endpoint: {}",
                &endpoint,
            );

            Connection::new(s)
        }
        Err(err) => {
            return Err(TransportInitError::ConnectionFail { source: err })
        }
    };

    let mut frame = Frame::array();
    frame.push_bulk(Bytes::from("power".as_bytes()));
    match conn.write_frame(&frame).await {
        Ok(_) => (),
        Err(err) => {
            println!("err: {}", err);
        }
    }

    Ok(())
}

pub(crate) async fn send_handshake_syn(
    ip: String,
    p2p_port: u16,
    // transport_meta: Arc<TransportMeta>,
) -> Result<(), TransportInitError> {
    // let my_p2p_port = transport_meta.my_p2p_port;
    // let active_calls = transport_meta.active_calls.clone();

    // let endpoint = format!("{}:{}", ip, p2p_port);

    // if active_calls.contain(&ip).await {
    //     return Err(TransportInitError::CallInProcess { ip });
    // }

    // conn.write_frame()

    // let my_sig = self.disc_state.id.sig();
    // let my_public_key_bytes = self.disc_state.id.public_key_bytes();

    // let way_syn =
    //     WhoAreYouSyn::new(my_sig, my_p2p_port, my_public_key_bytes);

    // let buf = match way_syn.to_bytes() {
    //     Ok(b) => b,
    //     Err(err) => {
    //         return Err(WhoareyouInitError::ByteConversionFail { err });
    //     }
    // };

    // self.udp_socket.send_to(&buf, endpoint.clone()).await?;

    // debug!(
    //     "Successfully sent WhoAreYou to endpoint: {}, buf len: {}",
    //     &endpoint,
    //     buf.len()
    // );

    Ok(())
}

pub(crate) fn is_my_endpoint(my_p2p_port: u16, endpoint: &String) -> bool {
    false
}
