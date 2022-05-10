use logger::twarn;
use p2p_active_calls::{ActiveCalls, Call, CallGuard};
use p2p_transport::{
    frame::{Error, Frame},
    parse::Parse,
};
use p2p_transport_handshake::ops::{Handshake, HANDSHAKE_SYN};
use std::net::SocketAddr;
use std::sync::Arc;
use thiserror::Error;

pub enum Request {
    HandshakeInit {
        msg: Handshake,
        call_guard: CallGuard,
    },
}

#[derive(Error, Debug)]
pub enum RequestParseError {
    #[error(
        "Frame is not an array frame (only one we support currently, \
        err: {err}"
    )]
    NotArrayFrame { err: String },

    #[error("Frame type (string) parse error, err: {err}")]
    FrameTypeParseError { err: String },

    #[error("Already in call with this client, call: {call}")]
    AlreadyInCall { call: Arc<Call> },

    #[error("Error parsing message, err: {err}")]
    MsgParseError { err: String },

    #[error("Unknown frame type, frame_type: {frame_type}")]
    UnknownFrameType { frame_type: String },
}

impl Request {
    pub async fn new(
        socket_addr: SocketAddr,
        frame: Frame,
        active_calls: Arc<ActiveCalls>,
    ) -> Result<Request, RequestParseError> {
        let mut parse = match Parse::new(frame) {
            Ok(p) => p,
            Err(err) => {
                return Err(RequestParseError::NotArrayFrame {
                    err: err.to_string(),
                });
            }
        };

        let frame_type = match parse.next_string() {
            Ok(t) => t,
            Err(err) => {
                return Err(RequestParseError::FrameTypeParseError {
                    err: err.to_string(),
                });
            }
        };

        match &frame_type[..] {
            HANDSHAKE_SYN => {
                let handshake = match Handshake::parse_frames(&mut parse) {
                    Ok(f) => f,
                    Err(err) => {
                        return Err(RequestParseError::MsgParseError {
                            err: err.to_string(),
                        });
                    }
                };

                let src_p2p_endpoint =
                    format!("{}:{}", socket_addr.ip(), handshake.src_p2p_port);

                let call_guard = {
                    match active_calls.get(&src_p2p_endpoint).await {
                        Some(call) => {
                            return Err(RequestParseError::AlreadyInCall {
                                call,
                            });
                        }
                        None => {
                            active_calls
                                .insert_inbound(src_p2p_endpoint.clone())
                                .await;

                            CallGuard {
                                endpoint: src_p2p_endpoint.clone(),
                                active_calls: active_calls.clone(),
                            }
                        }
                    }
                };

                let op = Request::HandshakeInit {
                    msg: handshake,
                    call_guard,
                };

                Ok(op)
            }
            _ => {
                return Err(RequestParseError::UnknownFrameType { frame_type });
            }
        }
    }
}
