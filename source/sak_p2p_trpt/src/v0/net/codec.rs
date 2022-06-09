use crate::{BoxedError, Handshake, Msg, SyncTx, SyncTxHash};
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
            // Msg::SyncTx(sync) => sync.into_frame(),
            Msg::SyncTxHash(sync_tx_hash) => sync_tx_hash.into_frame(),
            Msg::RequestTxs(sync_tx_hash) => sync_tx_hash.into_frame(),
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
                "hs_syn" => {
                    let handshake = Handshake::from_parse(&mut parse)?;
                    Msg::HandshakeSyn(handshake)
                }
                "hs_ack" => {
                    let handshake = Handshake::from_parse(&mut parse)?;
                    Msg::HandshakeAck(handshake)
                }
                // "sync_tx" => {
                //     let sync = SyncTx::from_parse(&mut parse)?;
                //     Msg::SyncTx(sync)
                // }
                "sync_tx_hash" => {
                    let sync = SyncTxHash::from_parse(&mut parse)?;
                    Msg::SyncTxHash(sync)
                }
                "request_txs" => {
                    let sync = SyncTxHash::from_parse(&mut parse)?;
                    Msg::SyncTxHash(sync)
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
