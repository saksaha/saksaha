// use logger::twarn;
// use p2p_transport::{
//     frame::{Error, Frame},
//     parse::Parse,
// };
// use p2p_transport_handshake::ops::{Handshake, HANDSHAKE_SYN};
// use std::net::SocketAddr;
// use std::sync::Arc;
// use thiserror::Error;

// pub enum Request {
//     HandshakeInit { msg: Handshake },
// }

// #[derive(Error, Debug)]
// pub enum RequestParseError {
//     #[error(
//         "Frame is not an array frame (only one we support currently, \
//         err: {err}"
//     )]
//     NotArrayFrame { err: String },

//     #[error("Frame type (string) parse error, err: {err}")]
//     FrameTypeParseError { err: String },

//     #[error("Error parsing message, err: {err}")]
//     MsgParseError { err: String },

//     #[error("Unknown frame type, frame_type: {frame_type}")]
//     UnknownFrameType { frame_type: String },
// }

// impl Request {
//     pub async fn new(
//         socket_addr: SocketAddr,
//         frame: Frame,
//     ) -> Result<Request, RequestParseError> {
//         let mut parse = match Parse::new(frame) {
//             Ok(p) => p,
//             Err(err) => {
//                 return Err(RequestParseError::NotArrayFrame {
//                     err: err.to_string(),
//                 });
//             }
//         };

//         let frame_type = match parse.next_string() {
//             Ok(t) => t,
//             Err(err) => {
//                 return Err(RequestParseError::FrameTypeParseError {
//                     err: err.to_string(),
//                 });
//             }
//         };

//         match &frame_type[..] {
//             HANDSHAKE_SYN => {
//                 let handshake = match Handshake::parse_frames(&mut parse) {
//                     Ok(f) => f,
//                     Err(err) => {
//                         return Err(RequestParseError::MsgParseError {
//                             err: err.to_string(),
//                         });
//                     }
//                 };

//                 let op = Request::HandshakeInit { msg: handshake };

//                 Ok(op)
//             }
//             _ => {
//                 return Err(RequestParseError::UnknownFrameType { frame_type });
//             }
//         }
//     }
// }
