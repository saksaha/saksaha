use crate::{
    Msg, TrptError, BLOCK_HASH_ACK, BLOCK_HASH_SYN, BLOCK_SYN_TYPE,
    HANDSHAKE_ACK_TYPE, HANDSHAKE_SYN_TYPE, PING_TYPE, TX_HASH_ACK_TYPE,
    TX_HASH_SYN_TYPE, TX_SYN_TYPE,
};
use bytes::BytesMut;
use sak_p2p_frame::frame_io;

pub(super) fn encode_into_frame(
    item: Msg,
    dst: &mut BytesMut,
) -> Result<&'static str, TrptError> {
    let (frame, msg_type) = match item {
        Msg::Ping(ping) => (ping.into_frame(), PING_TYPE),
        Msg::HandshakeSyn(handshake) => {
            (handshake.into_syn_frame(), HANDSHAKE_SYN_TYPE)
        }
        Msg::HandshakeAck(handshake) => {
            (handshake.into_ack_frame(), HANDSHAKE_ACK_TYPE)
        }
        Msg::TxSyn(sync) => (sync.into_frame(), TX_SYN_TYPE),
        Msg::TxHashSyn(sync_tx_hash) => {
            (sync_tx_hash.into_syn_frame(), TX_HASH_SYN_TYPE)
        }
        Msg::TxHashAck(sync_tx_hash) => {
            (sync_tx_hash.into_ack_frame(), TX_HASH_ACK_TYPE)
        }
        Msg::BlockHashSyn(block_hash_sync) => {
            (block_hash_sync.into_syn_frame(), BLOCK_HASH_SYN)
        }
        Msg::BlockHashAck(block_hash_sync) => {
            (block_hash_sync.into_ack_frame(), BLOCK_HASH_ACK)
        }
        Msg::BlockSyn(sync_block) => (sync_block.into_frame(), BLOCK_SYN_TYPE),
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

    Ok(msg_type)
}
