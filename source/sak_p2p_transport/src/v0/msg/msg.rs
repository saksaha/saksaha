use crate::{
    BlockAckMsg, BlockHashSyncMsg, BlockSynMsg, ErrorMsg, HandshakeMsg,
    HelloMsg, PingMsg, TxAckMsg, TxHashSyncMsg, TxSynMsg,
};

#[derive(Debug)]
pub enum Msg {
    HelloSyn(HelloMsg),

    HelloAck(HelloMsg),

    HandshakeSyn(HandshakeMsg),

    HandshakeAck(HandshakeMsg),

    TxHashSyn(TxHashSyncMsg),

    TxHashAck(TxHashSyncMsg),

    TxSyn(TxSynMsg),

    TxAck(TxAckMsg),

    BlockHashSyn(BlockHashSyncMsg),

    BlockHashAck(BlockHashSyncMsg),

    BlockSyn(BlockSynMsg),

    BlockAck(BlockAckMsg),

    Error(ErrorMsg),

    Ping(PingMsg),
}

impl std::fmt::Display for Msg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Msg::HelloSyn(_) => write!(f, "hello_syn"),
            Msg::HelloAck(_) => write!(f, "helloack"),
            Msg::HandshakeSyn(_) => write!(f, "handshake_syn"),
            Msg::HandshakeAck(_) => write!(f, "handshake_ack"),
            Msg::TxHashSyn(_) => write!(f, "tx_hash_syn"),
            Msg::TxHashAck(_) => write!(f, "tx_hash_ack"),
            Msg::TxSyn(_) => write!(f, "tx_syn"),
            Msg::TxAck(_) => write!(f, "tx_ack"),
            Msg::Error(_) => write!(f, "error"),
            Msg::BlockHashSyn(_) => write!(f, "block_hash_syn"),
            Msg::BlockHashAck(_) => write!(f, "block_hash_ack"),
            Msg::BlockSyn(_) => write!(f, "block_syn"),
            Msg::BlockAck(_) => write!(f, "block_ack"),
            Msg::Ping(_) => write!(f, "ping"),
        }
    }
}
