use super::msg::{WhoAreYouAckMsg, WhoAreYouMsg, SAKSAHA};
use crate::v1::active_calls::Traffic;
use crate::v1::ops::Opcode;
use crate::v1::table::TableNode;
use crate::v1::task_queue::TaskQueue;
use crate::v1::DiscState;
use crate::v1::{address::Address, table::Table};
use crypto::{Crypto, Signature, SigningKey};
use log::debug;
use std::convert::TryInto;
use std::sync::Arc;
use thiserror::Error;
use tokio::io::{AsyncWriteExt, Interest};
use tokio::net::{TcpStream, UdpSocket};

#[derive(Error, Debug)]
pub enum WhoAreYouRecvError {
    #[error("Couldn't parse WhoAreYou message, err: {0}")]
    MessageParseFail(String),
}

pub struct WhoAreYouReceiver {
    disc_state: Arc<DiscState>,
    task_queue: Arc<TaskQueue>,
}

impl WhoAreYouReceiver {
    pub fn new(
        disc_state: Arc<DiscState>,
        task_queue: Arc<TaskQueue>,
    ) -> WhoAreYouReceiver {
        WhoAreYouReceiver { disc_state, task_queue }
    }

    pub fn handle_who_are_you(
        &self,
        addr: Address,
        buf: &[u8],
    ) -> Result<(), WhoAreYouRecvError> {
        let msg = match WhoAreYouMsg::parse(buf) {
            Ok(m) => m,
            Err(err) => {
                return Err(WhoAreYouRecvError::MessageParseFail(err));
            }
        };

        println!("msg: ");
        // let len: usize = {
        //     let mut len_buf: [u8; 4] = [0; 4];
        //     len_buf.copy_from_slice(&buf[..4]);

        //     let len = u32::from_le_bytes(len_buf);
        //     let len: usize = match len.try_into() {
        //         Ok(l) => l,
        //         Err(err) => {
        //             return Err(WhoAreYouRecvError::LengthParseFail(
        //                 len,
        //                 err.to_string(),
        //             ));
        //         }
        //     };
        //     len
        // };

        // let mut public_key_bytes = {
        //     let b = [0; 65];

        // };

        // let _ = match stream.read_exact(&mut buf).await {
        //     Ok(l) => {
        //         if l == 0 {
        //             return Err(format!("Nothing to read, 0 byte"));
        //         }
        //         l
        //     }
        //     Err(err) => {
        //         return Err(format!("Error reading whoAreYou, err: {}", err));
        //     }
        // };

        // let opcode = Opcode::from(buf[0]);

        // let sig_len = len
        //     - 1 // kind
        //     - 2 // peer_op_bytes
        //     - 65; // public_key_bytes

        // let sig: Signature = match buf[1..1 + sig_len].try_into() {
        //     Ok(b) => {
        //         // log!(DEBUG, "Parsing signature: {:?}", b);

        //         match Signature::from_der(b) {
        //             Ok(s) => s,
        //             Err(err) => {
        //                 return Err(format!(
        //                     "Error recovering signature, err: {}",
        //                     err
        //                 ));
        //             }
        //         }
        //     }
        //     Err(err) => {
        //         return Err(format!("Error parsing signature, err: {}", err));
        //     }
        // };

        // let sig_end = 1 + sig_len;

        // let peer_op_port: u16 = match buf[sig_end..sig_end + 2].try_into() {
        //     Ok(p) => u16::from_be_bytes(p),
        //     Err(err) => {
        //         return Err(format!(
        //             "Error parsing peer_op_port, err: {}",
        //             err
        //         ));
        //     }
        // };

        // let peer_op_port_end = 1 + sig_len + 2;
        // let mut public_key_bytes = [0; 65];
        // public_key_bytes
        //     .copy_from_slice(&buf[peer_op_port_end..peer_op_port_end + 65]);

        // let mut way =
        //     WhoAreYouMsg::new(opcode, sig, peer_op_port, public_key_bytes);

        // let mut new_buf = len_buf.to_vec();
        // new_buf.extend_from_slice(&buf);
        // way.raw = new_buf;

        // Ok(way)
        Ok(())
    }
}
