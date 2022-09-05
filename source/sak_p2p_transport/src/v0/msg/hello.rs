use crate::{MsgType, TrptError};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_addr::UnknownAddr;
use sak_p2p_frame::{Frame, Parse};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct HelloMsg {
    pub unknown_addrs: Vec<UnknownAddr>,
}

impl HelloMsg {
    pub fn new(unknown_addrs: Vec<UnknownAddr>) -> Result<HelloMsg, String> {
        Ok(HelloMsg { unknown_addrs })
    }

    pub fn from_parse(parse: &mut Parse) -> Result<HelloMsg, TrptError> {
        let addr_count = parse.next_int()? as u16;

        let mut v = Vec::with_capacity(addr_count as usize);

        for _ in 0..addr_count {
            let ip: String = {
                let ip = parse.next_bytes()?;

                std::str::from_utf8(ip.as_ref())?.into()
            };

            let disc_port = parse.next_int()? as u16;

            let unknown_addr = UnknownAddr::new_from_endpoint(&ip, disc_port);

            v.push(unknown_addr)
        }

        let h = HelloMsg { unknown_addrs: v };

        Ok(h)
    }

    fn into_frame(&self, msg_type: &'static str) -> Frame {
        let mut frame = Frame::array();

        frame.push_bulk(Bytes::from(msg_type));

        let addr_count = self.unknown_addrs.len();
        frame.push_int(addr_count as u128);

        for idx in 0..addr_count {
            let unknown_addr = &self.unknown_addrs[idx];

            let ip = {
                let mut ip = BytesMut::new();

                ip.put(unknown_addr.ip.as_bytes());

                ip
            };

            frame.push_bulk(ip.into());

            frame.push_int(unknown_addr.disc_port as u128);
        }

        frame
    }

    pub fn into_syn_frame(&self) -> Frame {
        self.into_frame(MsgType::HELLO_SYN)
    }

    pub fn into_ack_frame(&self) -> Frame {
        self.into_frame(MsgType::HELLO_ACK)
    }
}
