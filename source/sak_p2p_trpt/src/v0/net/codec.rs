use crate::{
    BoxedError, Handshake, Msg, TxHashSyn, TxSyn, HANDSHAKE_ACK_TYPE,
    HANDSHAKE_SYN_TYPE, TX_HASH_ACK_TYPE, TX_HASH_SYN_TYPE, TX_SYN_TYPE,
};
use bytes::BytesMut;
use sak_p2p_frame::{frame_io, Parse};
use tokio_util::codec::{Decoder, Encoder};

pub struct P2PCodec {}

impl Encoder<Msg> for P2PCodec {
    type Error = BoxedError;

    fn encode(
        &mut self,
        item: Msg,
        dst: &mut BytesMut,
    ) -> Result<(), BoxedError> {
        let frame = match item {
            Msg::HandshakeSyn(handshake) => handshake.into_syn_frame(),
            Msg::HandshakeAck(handshake) => handshake.into_ack_frame(),
            Msg::TxSyn(sync) => sync.into_frame(),
            Msg::TxHashSyn(sync_tx_hash) => sync_tx_hash.into_syn_frame(),
            Msg::TxHashAck(sync_tx_hash) => sync_tx_hash.into_ack_frame(),
        };

        match frame_io::write_frame(dst, &frame) {
            Ok(_) => (),
            Err(err) => {
                return Err(format!(
                    "Error writing who_are_you_syn_frame, err: {}",
                    err
                )
                .into());
            }
        };

        return Ok(());
    }
}

impl Decoder for P2PCodec {
    type Item = Msg;
    type Error = BoxedError;

    fn decode(
        &mut self,
        src: &mut BytesMut,
    ) -> Result<Option<Self::Item>, BoxedError> {
        if let Some(frame) = frame_io::parse_frame(src)? {
            let mut parse = Parse::new(frame)?;

            let msg_type = parse.next_string()?.to_lowercase();

            let msg = match msg_type.as_str() {
                HANDSHAKE_SYN_TYPE => {
                    let handshake = Handshake::from_parse(&mut parse)?;
                    Msg::HandshakeSyn(handshake)
                }
                HANDSHAKE_ACK_TYPE => {
                    let handshake = Handshake::from_parse(&mut parse)?;
                    Msg::HandshakeAck(handshake)
                }
                TX_HASH_SYN_TYPE => {
                    let tx_hash_syn = TxHashSyn::from_parse(&mut parse)?;
                    Msg::TxHashSyn(tx_hash_syn)
                }
                TX_HASH_ACK_TYPE => {
                    let tx_hash_ack = TxHashSyn::from_parse(&mut parse)?;
                    Msg::TxHashAck(tx_hash_ack)
                }
                TX_SYN_TYPE => {
                    let tx_syn = TxSyn::from_parse(&mut parse)?;
                    Msg::TxSyn(tx_syn)
                }
                _ => {
                    return Err(format!(
                        "Frame does have invalid msg_type, type: {}",
                        msg_type
                    )
                    .into());
                }
            };

            return Ok(Some(msg));
        }

        Ok(None)
    }
}
