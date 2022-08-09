use crate::{
    BlockHashSynMsg, BlockSynMsg, Handshake, Msg, PingMsg, TrptError,
    TxHashSynMsg, TxSynMsg, BLOCK_HASH_ACK, BLOCK_HASH_SYN, BLOCK_SYN_TYPE,
    HANDSHAKE_ACK_TYPE, HANDSHAKE_SYN_TYPE, PING_TYPE, TX_HASH_ACK_TYPE,
    TX_HASH_SYN_TYPE, TX_SYN_TYPE,
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
            HANDSHAKE_SYN_TYPE => {
                let handshake = Handshake::from_parse(&mut parse)?;
                Msg::HandshakeSyn(handshake)
            }
            HANDSHAKE_ACK_TYPE => {
                let handshake = Handshake::from_parse(&mut parse)?;
                Msg::HandshakeAck(handshake)
            }
            TX_HASH_SYN_TYPE => {
                let tx_hash_syn = TxHashSynMsg::from_parse(&mut parse)?;
                Msg::TxHashSyn(tx_hash_syn)
            }
            TX_HASH_ACK_TYPE => {
                let tx_hash_ack = TxHashSynMsg::from_parse(&mut parse)?;
                Msg::TxHashAck(tx_hash_ack)
            }
            TX_SYN_TYPE => {
                let tx_sync = TxSynMsg::from_parse(&mut parse)?;
                Msg::TxSyn(tx_sync)
            }
            BLOCK_HASH_SYN => {
                let block_hash_sync = BlockHashSynMsg::from_parse(&mut parse)?;
                Msg::BlockHashSyn(block_hash_sync)
            }
            BLOCK_HASH_ACK => {
                let block_hash_ack = BlockHashSynMsg::from_parse(&mut parse)?;
                Msg::BlockHashAck(block_hash_ack)
            }
            BLOCK_SYN_TYPE => {
                let block_syn = BlockSynMsg::from_parse(&mut parse)?;
                Msg::BlockSyn(block_syn)
            }
            PING_TYPE => {
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
