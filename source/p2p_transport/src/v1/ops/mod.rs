pub mod handshake;

use super::parse::Parse;
use crate::{frame::Frame, Error};
pub use handshake::*;

pub const HANDSHAKE_INIT_OP: &'static str = "handshake_init";

pub enum Operation {
    HandshakeInit(Handshake),
}

impl Operation {
    pub fn from_frame(frame: Frame) -> Result<Operation, Error> {
        let mut parse = Parse::new(frame)?;

        let op_name = parse.next_string()?;
        println!("operation: {}", op_name);

        match &op_name[..] {
            HANDSHAKE_INIT_OP => {
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
