use p2p_transport::{
    frame::{Error, Frame},
    parse::Parse,
};
use p2p_transport_handshake::ops::Handshake;

pub const HANDSHAKE_SYN: &'static str = "handshake_syn";
pub const HANDSHAKE_ACK: &'static str = "handshake_ack";

pub enum Operation {
    HandshakeInit(Handshake),
}

impl Operation {
    pub fn from_frame(frame: Frame) -> Result<Operation, String> {
        let mut parse = match Parse::new(frame) {
            Ok(p) => p,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        let frame_type = match parse.next_string() {
            Ok(t) => t,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        match &frame_type[..] {
            HANDSHAKE_SYN => {
                let handshake_frame = match Handshake::parse_frames(&mut parse)
                {
                    Ok(f) => f,
                    Err(err) => {
                        return Err(format!(
                            "Error parsing HANDSHAKE_SYN frame, err: {}",
                            err
                        ));
                    }
                };

                let op = Operation::HandshakeInit(handshake_frame);

                Ok(op)
            }
            _ => {
                return Err(format!(
                    "Unknown operation. Frame type might be wrong,\
                    frame_type: {}",
                    frame_type,
                ));
            }
        }
    }
}
