use crate::{BlockHashSynMsg, BlockSynMsg, Handshake, TxHashSynMsg, TxSynMsg};

#[derive(Debug)]
pub enum Msg {
    HandshakeSyn(Handshake),
    HandshakeAck(Handshake),
    TxSyn(TxSynMsg),
    TxHashSyn(TxHashSynMsg),
    TxHashAck(TxHashSynMsg),
    // HeightSyn(BlockHeightSync),
    // HeightAck(BlockHeightSync),
    BlockHashSyn(BlockHashSynMsg),
    BlockHashAck(BlockHashSynMsg),
    BlockSyn(BlockSynMsg),
}
