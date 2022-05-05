use super::check;
use crate::connection::Connection;
use crate::msg::Handshake;
use logger::tdebug;
use p2p_identity::addr::KnownAddr;
use thiserror::Error;
use tokio::net::TcpStream;

pub struct HandshakeInitArgs {
    pub p2p_port: u16,
    pub addr: KnownAddr,
}

#[derive(Error, Debug)]
pub enum HandshakeInitError {
    #[error("P2P Port may not be provided")]
    InvalidP2PEndpoint,

    #[error("Cannot send request to myself, addr: {addr}")]
    MyEndpoint { addr: KnownAddr },

    #[error("Cannot create tcp stream into endpoint, err: {err}")]
    ConnectionFail { err: String },

    #[error("Cannot retrieve peer address, err: {err}")]
    PeerAddressNotRetrievable { err: String },
}

pub async fn initiate_handshake(
    handshake_init_args: HandshakeInitArgs,
) -> Result<(), HandshakeInitError> {
    let HandshakeInitArgs { p2p_port, addr, .. } = handshake_init_args;

    let endpoint = addr.p2p_endpoint();
    if check::is_my_endpoint(p2p_port, &endpoint) {
        return Err(HandshakeInitError::MyEndpoint { addr });
    }

    let mut conn = match TcpStream::connect(endpoint).await {
        Ok(s) => {
            let socket_addr = match s.peer_addr() {
                Ok(a) => a,
                Err(err) => {
                    return Err(
                        HandshakeInitError::PeerAddressNotRetrievable {
                            err: err.to_string(),
                        },
                    );
                }
            };

            tdebug!(
                "p2p_tranposrt",
                "handshake",
                "Call success, addr: {:?}",
                socket_addr,
            );

            Connection::new(s)
        }
        Err(err) => {
            return Err(HandshakeInitError::ConnectionFail {
                err: err.to_string(),
            })
        }
    };

    let _p = addr.public_key;
    // println!("power: {:?}", p);

    // let dest_pub_key = addr.public_key

    // let handshake = Handshake {
    //     src_public_key: identity.public_key,
    //     dst_public_key: her_public_key,
    // };

    // match send_handshake_msg(&mut stream, handshake_msg).await {
    //     Ok(_) => (),
    //     Err(err) => {
    //         return Err(HandshakeInitError::PayloadWriteFail {
    //             err: err.to_string(),
    //         })
    //     }
    // };

    // let mut hs_ack_buf = read_handshake_msg(&mut stream).await?;

    // let hs_msg = match parse_handshake_msg(
    //     &mut hs_ack_buf,
    //     &mut stream,
    //     identity.clone(),
    // ) {
    //     Ok(s) => s,
    //     Err(err) => {
    //         return Err(HandshakeInitError::Invalid { err });
    //     }
    // };

    // let shared_secret =
    //     match make_shared_secret(identity.clone(), hs_msg.dst_public_key) {
    //         Ok(s) => s,
    //         Err(err) => return Err(HandshakeInitError::Invalid { err }),
    //     };

    // debug!(
    //     "Successfully initiated handshake, peer: {:?}",
    //     stream.peer_addr()
    // );

    // let t = Transport {
    //     stream,
    //     shared_secret,
    //     peer_id: hs_msg.dst_public_key,
    // };

    // Ok(t)

    Ok(())
}

// use crate::{Connection, Frame, Transport};
// use bytes::{Buf, BufMut, Bytes, BytesMut};
// use crypto::{
//     EncodedPoint, EphemeralSecret, PublicKey, Secp256k1, SharedSecret,
// };
// use log::{debug, error};
// use p2p_identity::{P2PIdentity, PeerId};
// use rand::rngs::OsRng;
// use std::convert::TryInto;
// use std::{
//     io::{Cursor, Seek, SeekFrom, Write},
//     sync::Arc,
// };
// use thiserror::Error;
// use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};
// use tokio::net::TcpStream;

// pub const HANDSHAKE_CODE: u8 = b'#';

// #[derive(Error, Debug)]
// pub enum HandshakeInitError {
//     #[error("Request to my (recursive) endpoint: {endpoint}")]
//     MyEndpoint { endpoint: String },

//     #[error("Can't connect to endpoint")]
//     ConnectionFail {
//         #[from]
//         source: std::io::Error,
//     },

//     #[error("Already talking: {ip}")]
//     CallInProcess { ip: String },

//     #[error("Can't write payload into buffer")]
//     PayloadWriteFail { err: String },

//     #[error("Can't read ack msg: {endpoint}")]
//     InvalidAck { endpoint: String },

//     #[error("Can't send handshake msg to endpoint: {endpoint}")]
//     HandshakeSentFail { endpoint: String },

//     #[error("Can't receive handshake")]
//     Invalid { err: String },
// }

// #[derive(Error, Debug)]
// pub enum HandshakeRecvError {
//     #[error("Can't receive handshake")]
//     Invalid { err: String },

//     #[error("Can't read stream")]
//     CannotReadStream {
//         #[from]
//         source: std::io::Error,
//     },
// }

// pub struct HandshakeMsg {
//     src_public_key: PeerId,
//     dst_public_key: PeerId,
// }

// #[derive(Clone)]
// pub struct HandshakeInitParams {
//     pub identity: Arc<P2PIdentity>,
//     pub my_rpc_port: u16,
//     pub my_p2p_port: u16,
//     pub her_ip: String,
//     pub her_p2p_port: u16,
//     pub her_public_key: PeerId,
// }

// pub async fn receive_handshake(
//     mut stream: TcpStream,
//     identity: Arc<P2PIdentity>,
// ) -> Result<Transport, HandshakeRecvError> {
//     let mut hs_buf = read_handshake_msg(&mut stream).await?;

//     let hs_msg =
//         match parse_handshake_msg(&mut hs_buf, &mut stream, identity.clone()) {
//             Ok(s) => s,
//             Err(err) => {
//                 return Err(HandshakeRecvError::Invalid { err });
//             }
//         };

//     // if identity.public_key == hs_msg.src_public_key {
//     //     println!("222");
//     //     return Err(HandshakeRecvError::Invalid {
//     //         err: "my identity".to_string(),
//     //     });
//     // }

//     let hs_ack_msg = HandshakeMsg {
//         src_public_key: identity.public_key,
//         dst_public_key: hs_msg.src_public_key,
//     };

//     match send_handshake_msg(&mut stream, hs_ack_msg).await {
//         Ok(_) => (),
//         Err(err) => {
//             return Err(HandshakeRecvError::Invalid {
//                 err: err.to_string(),
//             })
//         }
//     };

//     let shared_secret =
//         match make_shared_secret(identity.clone(), hs_msg.src_public_key) {
//             Ok(s) => s,
//             Err(err) => return Err(HandshakeRecvError::Invalid { err }),
//         };

//     debug!(
//         "Successfully received handshake, endpoint: {:?}",
//         stream.peer_addr()
//     );

//     Ok(Transport {
//         stream,
//         shared_secret,
//         peer_id: hs_msg.src_public_key,
//     })
// }

// pub async fn initiate_handshake(
//     hs_init_params: HandshakeInitParams,
// ) -> Result<Transport, HandshakeInitError> {
//     let HandshakeInitParams {
//         my_rpc_port,
//         my_p2p_port,
//         her_ip,
//         her_p2p_port,
//         her_public_key,
//         identity,
//         ..
//     } = hs_init_params;

//     let endpoint = format!("{}:{}", her_ip.clone(), her_p2p_port);

//     if super::is_my_endpoint(my_p2p_port, &endpoint) {
//         return Err(HandshakeInitError::MyEndpoint { endpoint });
//     }

//     // if her_public_key == identity.public_key {
//     //     println!("111");
//     //     return Err(HandshakeInitError::Invalid {
//     //         err: "my endpoint".to_string(),
//     //     });
//     // }

//     let mut stream = match TcpStream::connect(&endpoint).await {
//         Ok(s) => {
//             debug!(
//                 "Transport handshake: Successfully connected to endpoint: {}",
//                 &endpoint,
//             );
//             s
//         }
//         Err(err) => {
//             return Err(HandshakeInitError::ConnectionFail { source: err })
//         }
//     };

//     let handshake_msg = HandshakeMsg {
//         src_public_key: identity.public_key,
//         dst_public_key: her_public_key,
//     };

//     match send_handshake_msg(&mut stream, handshake_msg).await {
//         Ok(_) => (),
//         Err(err) => {
//             return Err(HandshakeInitError::PayloadWriteFail {
//                 err: err.to_string(),
//             })
//         }
//     };

//     let mut hs_ack_buf = read_handshake_msg(&mut stream).await?;

//     let hs_msg = match parse_handshake_msg(
//         &mut hs_ack_buf,
//         &mut stream,
//         identity.clone(),
//     ) {
//         Ok(s) => s,
//         Err(err) => {
//             return Err(HandshakeInitError::Invalid { err });
//         }
//     };

//     let shared_secret =
//         match make_shared_secret(identity.clone(), hs_msg.dst_public_key) {
//             Ok(s) => s,
//             Err(err) => return Err(HandshakeInitError::Invalid { err }),
//         };

//     debug!(
//         "Successfully initiated handshake, peer: {:?}",
//         stream.peer_addr()
//     );

//     let t = Transport {
//         stream,
//         shared_secret,
//         peer_id: hs_msg.dst_public_key,
//     };

//     Ok(t)
// }

// async fn read_handshake_msg(
//     stream: &mut TcpStream,
// ) -> Result<BytesMut, std::io::Error> {
//     let mut buffer = BytesMut::with_capacity(512);

//     let n = stream.read_buf(&mut buffer).await?;

//     // println!("buf received: {:?}", buffer.to_vec());

//     Ok(buffer)
// }

// fn check_msg(buffer: &BytesMut) -> Result<(), String> {
//     if buffer.len() > 0 {
//         let code = buffer[0];

//         match code {
//             HANDSHAKE_CODE => {
//                 return Ok(());
//             }
//             _ => {
//                 return Err(format!(
//                     "Listener currently takes only 'handshake' msg"
//                 ))
//             }
//         }
//     } else {
//         return Err(format!("Msg too short"));
//     }
// }

// #[cfg(target_pointer_width = "64")]
// async fn send_handshake_msg(
//     stream: &mut TcpStream,
//     hs_msg: HandshakeMsg,
// ) -> Result<(), std::io::Error> {
//     let mut buf = Cursor::new(Vec::new());

//     std::io::Write::write(&mut buf, &hs_msg.src_public_key[..])?;
//     std::io::Write::write(&mut buf, &hs_msg.dst_public_key[..])?;
//     let len = buf.position().to_le_bytes();

//     println!("sending: {:?}", &buf.get_ref()[..]);

//     stream.write_u8(HANDSHAKE_CODE).await?;
//     stream.write_all(&len[..]).await?;
//     stream.write_all(&buf.get_ref()[..]).await?;
//     stream.write_all(b"\r\n").await?;

//     // debug!("Sent handshake msg: {:?}", buf);

//     Ok(())
// }

// #[cfg(target_pointer_width = "64")]
// fn parse_handshake_msg(
//     hs_buf: &mut BytesMut,
//     stream: &mut TcpStream,
//     identity: Arc<P2PIdentity>,
// ) -> Result<HandshakeMsg, String> {
//     match check_msg(&hs_buf) {
//         Ok(_) => (),
//         Err(err) => return Err(format!("Invalid handshake msg")),
//     }

//     debug!("Checking msg: {:?}", hs_buf.to_vec());

//     let mut buf = Cursor::new(&hs_buf[..]);

//     buf.advance(1);

//     if !buf.has_remaining() {
//         error!("Msg is too short1");
//         return Err(format!("Msg too short"));
//     }

//     let mut len_buf = Bytes::copy_from_slice(&buf.chunk()[..8]);
//     let len = len_buf.get_u64_le() as usize;

//     buf.advance(8);

//     if !buf.has_remaining() {
//         error!("Msg is too short2");
//         return Err(format!("Msg too short"));
//     }

//     let data = Bytes::copy_from_slice(&buf.chunk()[..len]);

//     let dst_public_key = Bytes::copy_from_slice(&data[..65]);
//     let dst_public_key = match dst_public_key[..65].try_into() {
//         Ok(k) => k,
//         Err(e) => return Err(format!("Cannot create public key")),
//     };

//     Ok(HandshakeMsg {
//         src_public_key: identity.public_key,
//         dst_public_key: dst_public_key,
//     })
// }

// fn make_shared_secret(
//     identity: Arc<P2PIdentity>,
//     her_public_key: PeerId,
// ) -> Result<SharedSecret<Secp256k1>, String> {
//     let my_secret_key = &identity.secret_key;

//     let her_public =
//         match PublicKey::<Secp256k1>::from_sec1_bytes(&her_public_key) {
//             Ok(p) => p,
//             Err(err) => {
//                 return Err(format!("Cannot create her public key"));
//             }
//         };

//     let shared_secret = crypto::make_shared_secret(my_secret_key, her_public);

//     Ok(shared_secret)
// }

// pub(crate) fn is_my_endpoint(my_p2p_port: u16, endpoint: &String) -> bool {
//     false
// }
