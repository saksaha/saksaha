use crate::{Msg, MsgType, TrptError};
use bytes::BytesMut;
use sak_p2p_frame::frame_io;

pub(crate) fn encode_into_frame(item: Msg, dst: &mut BytesMut) -> Result<&'static str, TrptError> {
    let (frame, msg_type) = match item {
        Msg::Ping(ping) => (ping.into_frame(), MsgType::PING),
        Msg::HelloSyn(hello) => (hello.into_syn_frame(), MsgType::HELLO_SYN),
        Msg::HelloAck(hello) => (hello.into_ack_frame(), MsgType::HELLO_SYN),
        Msg::HandshakeSyn(handshake) => (handshake.into_syn_frame(), MsgType::HANDSHAKE_SYN),
        Msg::HandshakeAck(handshake) => (handshake.into_ack_frame(), MsgType::HANDSHAKE_ACK),
        Msg::TxSyn(sync) => (sync.into_frame(), MsgType::TX_SYN),
        Msg::TxAck(m) => (m.into_frame(), MsgType::TX_ACK),
        Msg::TxHashSyn(sync_tx_hash) => (sync_tx_hash.into_syn_frame(), MsgType::TX_HASH_SYN),
        Msg::TxHashAck(sync_tx_hash) => (sync_tx_hash.into_ack_frame(), MsgType::TX_HASH_ACK),
        Msg::BlockHashSyn(block_hash_sync) => {
            (block_hash_sync.into_syn_frame(), MsgType::BLOCK_HASH_SYN)
        }
        Msg::BlockHashAck(block_hash_sync) => {
            (block_hash_sync.into_ack_frame(), MsgType::BLOCK_HASH_ACK)
        }
        Msg::BlockSyn(sync_block) => (sync_block.into_frame(), MsgType::BLOCK_SYN),
        Msg::BlockAck(m) => (m.into_frame(), MsgType::BLOCK_ACK),
        Msg::Error(error) => (error.into_frame(), MsgType::ERROR),
    };

    match frame_io::write_frame(dst, &frame) {
        Ok(_) => (),
        Err(err) => {
            return Err(format!("Error writing who_are_you_syn_frame, err: {}", err).into());
        }
    };

    Ok(msg_type)
}
