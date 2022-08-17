use crate::{
    BlockAckMsg, BlockHashSyncMsg, BlockSynMsg, HandshakeMsg, Msg, MsgType,
    PingMsg, TrptError, TxAckMsg, TxHashSyncMsg, TxSynMsg,
};
use bytes::BytesMut;
use sak_p2p_frame::{frame_io, Parse};

pub(super) fn decode_into_msg(
    src: &mut BytesMut,
) -> Result<Option<Msg>, TrptError> {
    let maybe_frame = match frame_io::parse_frame(src) {
        Ok(f) => f,
        Err(err) => {
            return Err(format!(
                "Error parsing the frame, original_err: {}",
                err
            )
            .into())
        }
    };

    if let Some(frame) = maybe_frame {
        let mut parse = Parse::new(frame)?;

        let msg_type = parse.next_string()?.to_lowercase();

        let msg = match msg_type.as_str() {
            MsgType::HANDSHAKE_SYN => {
                let handshake = HandshakeMsg::from_parse(&mut parse)?;
                Msg::HandshakeSyn(handshake)
            }
            MsgType::HANDSHAKE_ACK => {
                let handshake = HandshakeMsg::from_parse(&mut parse)?;
                Msg::HandshakeAck(handshake)
            }
            MsgType::TX_HASH_SYN => {
                let tx_hash_sync = TxHashSyncMsg::from_parse(&mut parse)?;
                Msg::TxHashSyn(tx_hash_sync)
            }
            MsgType::TX_HASH_ACK => {
                let tx_hash_sync = TxHashSyncMsg::from_parse(&mut parse)?;
                Msg::TxHashAck(tx_hash_sync)
            }
            MsgType::TX_SYN => {
                let tx_syn = TxSynMsg::from_parse(&mut parse)?;
                Msg::TxSyn(tx_syn)
            }
            MsgType::TX_ACK => {
                let tx_ack = TxAckMsg::from_parse(&mut parse)?;
                Msg::TxAck(tx_ack)
            }
            MsgType::BLOCK_HASH_SYN => {
                let block_hash_sync = BlockHashSyncMsg::from_parse(&mut parse)?;
                Msg::BlockHashSyn(block_hash_sync)
            }
            MsgType::BLOCK_HASH_ACK => {
                let block_hash_ack = BlockHashSyncMsg::from_parse(&mut parse)?;
                Msg::BlockHashAck(block_hash_ack)
            }
            MsgType::BLOCK_SYN => {
                let block_syn = BlockSynMsg::from_parse(&mut parse)?;
                Msg::BlockSyn(block_syn)
            }
            MsgType::BLOCK_ACK => {
                let block_ack = BlockAckMsg::from_parse(&mut parse)?;
                Msg::BlockAck(block_ack)
            }
            MsgType::PING => {
                let ping = PingMsg::from_parse(&mut parse)?;
                Msg::Ping(ping)
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
    } else {
        return Ok(None);
    }
}
