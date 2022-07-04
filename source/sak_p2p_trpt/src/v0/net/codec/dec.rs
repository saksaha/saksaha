use crate::{
    BlockHashSynMsg, BlockSynMsg, BoxedError, Handshake, Msg, TxHashSynMsg,
    TxSynMsg, BLOCK_HASH_ACK, BLOCK_HASH_SYN, BLOCK_HEIGHT_SYN_TYPE,
    BLOCK_SYN_TYPE, HANDSHAKE_ACK_TYPE, HANDSHAKE_SYN_TYPE, TX_HASH_ACK_TYPE,
    TX_HASH_SYN_TYPE, TX_SYN_TYPE,
};
use bytes::BytesMut;
use sak_p2p_frame::{frame_io, Parse};

pub(super) fn decode_into_msg(
    src: &mut BytesMut,
) -> Result<Option<Msg>, BoxedError> {
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
            // BLOCK_HEIGHT_SYN_TYPE => {
            //     let tx_height = BlockHeightSynMsg::from_parse(&mut parse)?;
            //     Msg::BlockHeightSyn(tx_height)
            // }
            BLOCK_HASH_SYN => {
                let block_hash_sync = BlockHashSynMsg::from_parse(&mut parse)?;
                Msg::BlockHashSyn(block_hash_sync)
            }
            BLOCK_HASH_ACK => {
                let block_hash_sync = BlockHashSynMsg::from_parse(&mut parse)?;
                Msg::BlockHashAck(block_hash_sync)
            }

            BLOCK_SYN_TYPE => {
                let block_syn = BlockSynMsg::from_parse(&mut parse)?;
                Msg::BlockSyn(block_syn)
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
