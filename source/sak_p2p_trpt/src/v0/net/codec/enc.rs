use crate::{BoxedError, Msg};
use bytes::BytesMut;
use sak_p2p_frame::frame_io;

pub(super) fn encode_into_frame(
    item: Msg,
    dst: &mut BytesMut,
) -> Result<(), BoxedError> {
    let frame = match &item {
        Msg::HandshakeSyn(handshake) => handshake.into_syn_frame(),
        Msg::HandshakeAck(handshake) => handshake.into_ack_frame(),
        Msg::TxSyn(sync) => sync.into_frame(),
        Msg::TxHashSyn(sync_tx_hash) => sync_tx_hash.into_syn_frame(),
        Msg::TxHashAck(sync_tx_hash) => sync_tx_hash.into_ack_frame(),
        Msg::BlockHashSyn(block_hash_sync) => block_hash_sync.into_syn_frame(),
        Msg::BlockHashAck(block_hash_sync) => block_hash_sync.into_ack_frame(),
        Msg::BlockSyn(sync_block) => sync_block.into_frame(),
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

    Ok(())
}
