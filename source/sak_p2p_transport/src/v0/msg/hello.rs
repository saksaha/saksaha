use crate::{MsgType, TrptError};
use bytes::{BufMut, Bytes, BytesMut};
use sak_p2p_addr::UnknownAddr;
use sak_p2p_frame::{Frame, Parse};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct HelloMsg {
    pub unknown_addrs: Vec<UnknownAddr>,
    // pub instance_id: String,
    // pub src_p2p_port: u16,
    // pub src_public_key_str: String,
    // pub dst_public_key_str: String,
}

impl HelloMsg {
    pub fn new(
        unknown_addrs: Vec<UnknownAddr>,
        // src_p2p_port: u16,
        // src_public_key_str: String,
        // dst_public_key_str: String,
    ) -> Result<HelloMsg, String> {
        // let since_the_epoch = match SystemTime::now().duration_since(UNIX_EPOCH)
        // {
        //     Ok(s) => s,
        //     Err(err) => {
        //         return Err(format!("Couldn't get timestamp, err: {}", err))
        //     }
        // };

        // let instance_id = format!(
        //     "{}_{}",
        //     &src_public_key_str[124..],
        //     since_the_epoch.as_micros() % 100000,
        // );

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

        // let src_p2p_port = {
        //     let p = parse.next_int()? as u16;
        //     p
        // };

        // let instance_id = {
        //     let k = parse.next_bytes()?;
        //     std::str::from_utf8(k.as_ref())?.into()
        // };

        // let src_public_key_str: String = {
        //     let k = parse.next_bytes()?;
        //     std::str::from_utf8(k.as_ref())?.into()
        // };

        // let dst_public_key_str: String = {
        //     let k = parse.next_bytes()?;
        //     std::str::from_utf8(k.as_ref())?.into()
        // };

        let h = HelloMsg {
            unknown_addrs: v,
            // instance_id,
            // src_p2p_port,
            // src_public_key_str,
            // dst_public_key_str,
        };

        Ok(h)
    }

    fn into_frame(&self, msg_type: &'static str) -> Frame {
        let mut frame = Frame::array();

        // let instance_id_bytes = {
        //     let mut b = BytesMut::new();
        //     b.put(self.instance_id.as_bytes());
        //     b
        // };

        // let src_public_key_bytes = {
        //     let mut b = BytesMut::new();
        //     b.put(self.src_public_key_str.as_bytes());
        //     b
        // };

        // let dst_public_key_bytes = {
        //     let mut b = BytesMut::new();
        //     b.put(self.dst_public_key_str.as_bytes());
        //     b
        // };

        frame.push_bulk(Bytes::from(msg_type));

        // addr count
        let addr_count = self.unknown_addrs.len();
        frame.push_int(addr_count as u128);

        for idx in 0..addr_count {
            // frame.push_bulk(ip);
            // frame.push_int(disc_port);

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
