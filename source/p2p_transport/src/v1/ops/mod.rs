pub mod handshake;

use super::parse::Parse;
use crate::{frame::Frame, Error};
pub use handshake::*;

pub const HANDSHAKE_SYN: &'static str = "handshake_syn";
pub const HANDSHAKE_ACK: &'static str = "handshake_ack";

pub enum Operation {
    HandshakeInit(Handshake),
}

impl Operation {
    pub fn from_frame(frame: Frame) -> Result<Operation, Error> {
        let mut parse = Parse::new(frame)?;

        let frame_type = parse.next_string()?;

        match &frame_type[..] {
            HANDSHAKE_SYN => {
                let op = Operation::HandshakeInit(Handshake::parse_frames(
                    &mut parse,
                )?);

                Ok(op)
            }
            _ => {
                let e: Error = "str".into();
                return Err(e);
            }
        }
    }
}
