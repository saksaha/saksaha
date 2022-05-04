mod msg;

use k256::ecdsa::Signature;
pub(crate) use msg::{Msg, MsgType};

const SIG_LEN: usize = 71;

pub(crate) struct WhoAreYou {
    pub(crate) src_sig: Signature,
    pub(crate) src_disc_port: u16,
    pub(crate) src_p2p_port: u16,
    pub(crate) src_public_key_str: String,
}

impl WhoAreYou {
    pub fn into_syn_msg(&self) -> Result<Msg, String> {
        self.into_msg(MsgType::WhoAreYouSyn)
    }

    pub fn into_ack_msg(&self) -> Result<Msg, String> {
        self.into_msg(MsgType::WhoAreYouAck)
    }

    fn into_msg(&self, msg_type: MsgType) -> Result<Msg, String> {
        let mut buf: Vec<u8> = vec![];

        let src_p2p_port_bytes = self.src_p2p_port.to_be_bytes();
        let src_sig_bytes = {
            let s = self.src_sig.to_der().to_bytes();
            if s.len() != SIG_LEN {
                return Err(format!(
                    "Signature invalid length, expected: {}, len: {}",
                    SIG_LEN,
                    s.len()
                ));
            }
            s
        };
        let src_disc_port_bytes = self.src_disc_port.to_be_bytes();
        let src_public_key_bytes = self.src_public_key_str.as_bytes();

        buf.extend_from_slice(&src_p2p_port_bytes);
        buf.extend_from_slice(&src_sig_bytes);
        buf.extend_from_slice(&src_disc_port_bytes);
        buf.extend_from_slice(&src_public_key_bytes);

        let msg = Msg {
            msg_type,
            content: buf,
        };

        Ok(msg)
    }

    pub fn from_msg(msg: &Msg) -> Result<Self, String> {
        let content = &msg.content;

        let src_p2p_port = {
            let mut p2p_port_bytes = [0u8; 2];
            p2p_port_bytes.copy_from_slice(&content[0..2]);

            u16::from_be_bytes(p2p_port_bytes)
        };

        let src_sig = {
            let mut sig_bytes = [0u8; SIG_LEN];
            sig_bytes.copy_from_slice(&content[2..73]);

            let s = match Signature::from_der(&sig_bytes) {
                Ok(s) => s,
                Err(err) => {
                    return Err(format!(
                        "Error parsing signature from byte array, err: {}",
                        err
                    ));
                }
            };
            s
        };

        let src_disc_port = {
            let mut disc_port_bytes = [0u8; 2];
            disc_port_bytes.copy_from_slice(&content[73..75]);

            u16::from_be_bytes(disc_port_bytes)
        };

        let src_public_key_str = {
            let mut public_key_bytes = [0u8; 130];
            public_key_bytes.copy_from_slice(&content[75..205]);

            let s = match String::from_utf8(public_key_bytes.to_vec()) {
                Ok(s) => s,
                Err(err) => {
                    return Err(format!(
                        "Error parsing public key from byte array, err: {}",
                        err,
                    ));
                }
            };

            s
        };

        Ok(WhoAreYou {
            src_p2p_port,
            src_sig,
            src_disc_port,
            src_public_key_str,
        })
    }
}
