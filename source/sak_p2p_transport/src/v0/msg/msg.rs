use crate::{
    BlockHashSynMsg, BlockSynMsg, Handshake, PingMsg, TxHashSynMsg, TxSynMsg,
};

#[derive(Debug)]
pub enum Msg {
    HandshakeSyn(Handshake),

    HandshakeAck(Handshake),

    TxSyn(TxSynMsg),

    TxHashSyn(TxHashSynMsg),

    TxHashAck(TxHashSynMsg),

    BlockSyn(BlockSynMsg),

    BlockHashSyn(BlockHashSynMsg),

    BlockHashAck(BlockHashSynMsg),

    Ping(PingMsg),
}

impl Msg {
    pub fn name(&self) -> String {
        match &self {
            Msg::HandshakeSyn(_) => String::from("handshake_syn"),
            Msg::HandshakeAck(_) => String::from("handshake_ack"),
            Msg::TxSyn(_) => String::from("tx_syn"),
            Msg::TxHashSyn(_) => String::from("tx_hash_syn"),
            Msg::TxHashAck(_) => String::from("tx_hash_ack"),
            Msg::BlockSyn(_) => String::from("block_syn"),
            Msg::BlockHashSyn(_) => String::from("block_hash_syn"),
            Msg::BlockHashAck(_) => String::from("block_hash_ack"),
            Msg::Ping(_) => String::from("ping"),
        }
    }
}
