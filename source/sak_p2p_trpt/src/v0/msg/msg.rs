use crate::{
    BlockHashSynMsg, BlockHeightSynMsg, BlockSynMsg, Handshake, TxHashSynMsg,
    TxSynMsg,
};

#[derive(Debug)]
pub enum Msg {
    HandshakeSyn(Handshake),
    HandshakeAck(Handshake),
    TxSyn(TxSynMsg),
    TxHashSyn(TxHashSynMsg),
    TxHashAck(TxHashSynMsg),

    BlockHeightSyn(BlockHeightSynMsg),
    BlockHeightAck(BlockHeightSynMsg),

    // HeightSyn(BlockHeightSync),
    // HeightAck(BlockHeightSync),
    BlockHashSyn(BlockHashSynMsg),
    BlockHashAck(BlockHashSynMsg),
    BlockSyn(BlockSynMsg),
}
