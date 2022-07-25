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
