use crate::{
    BlockHashSynMsg, BlockSynMsg, Handshake, TxHashSynMsg, TxHeightSynMsg,
    TxSynMsg,
};

#[derive(Debug)]
pub enum Msg {
    HandshakeSyn(Handshake),
    HandshakeAck(Handshake),
    TxSyn(TxSynMsg),
    TxHashSyn(TxHashSynMsg),
    TxHashAck(TxHashSynMsg),

    TxHeightSyn(TxHeightSynMsg),
    TxHeightAck(TxHeightSynMsg),

    // HeightSyn(BlockHeightSync),
    // HeightAck(BlockHeightSync),
    BlockHashSyn(BlockHashSynMsg),
    BlockHashAck(BlockHashSynMsg),
    BlockSyn(BlockSynMsg),
}
