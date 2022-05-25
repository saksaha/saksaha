use crate::{
    v1::{
        net::{Frame, Parse},
        ops::{WHO_ARE_YOU_ACK_TYPE, WHO_ARE_YOU_SYN_TYPE},
    },
    BoxedError,
};
use bytes::{BufMut, Bytes, BytesMut};
use k256::ecdsa::Signature;

pub(crate) struct WhoAreYou {
    pub(crate) src_sig: Signature,
    pub(crate) src_disc_port: u16,
    pub(crate) src_p2p_port: u16,
    pub(crate) src_public_key_str: String,
}

impl WhoAreYou {
    pub(crate) fn into_syn_frame(&self) -> Result<Frame, String> {
        self.into_frame(WHO_ARE_YOU_SYN_TYPE)
    }

    pub(crate) fn into_ack_frame(&self) -> Result<Frame, String> {
        self.into_frame(WHO_ARE_YOU_ACK_TYPE)
    }

    fn into_frame(&self, msg_type: &'static str) -> Result<Frame, String> {
        let src_sig_bytes = {
            let mut b = BytesMut::new();
            b.put(&self.src_sig.to_der().to_bytes()[..]);
            b
        };

        let src_public_key_bytes = {
            let mut b = BytesMut::new();
            b.put(&self.src_public_key_str.as_bytes()[..]);
            b
        };

        let mut frame = Frame::array();
        frame.push_bulk(Bytes::from(msg_type.as_bytes()));
        frame.push_int(self.src_p2p_port as u64);
        frame.push_bulk(src_sig_bytes.into());
        frame.push_int(self.src_disc_port as u64);
        frame.push_bulk(src_public_key_bytes.into());

        Ok(frame)
    }

    pub(crate) fn parse_frames(
        parse: &mut Parse,
    ) -> Result<WhoAreYou, BoxedError> {
        let src_p2p_port = parse.next_int()? as u16;

        let src_sig = {
            let sig_bytes = parse.next_bytes()?;

            let s = match Signature::from_der(&sig_bytes) {
                Ok(s) => s,
                Err(err) => {
                    return Err(format!(
                        "Error parsing signature from byte array, err: {}",
                        err
                    )
                    .into());
                }
            };
            s
        };

        let src_disc_port = parse.next_int()? as u16;

        let src_public_key_str = {
            let src_public_key_bytes = parse.next_bytes()?;

            let s = match String::from_utf8(src_public_key_bytes.to_vec()) {
                Ok(s) => s,
                Err(err) => {
                    return Err(format!(
                        "Error parsing public key from byte array, err: {}",
                        err,
                    )
                    .into());
                }
            };

            s
        };

        parse.finish()?;

        let way = WhoAreYou {
            src_p2p_port,
            src_sig,
            src_disc_port,
            src_public_key_str,
        };

        return Ok(way);
    }
}
