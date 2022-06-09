use super::sync::SyncTxHash;
use crate::{Handshake, SyncTx};

pub enum Msg {
    HandshakeSyn(Handshake),
    HandshakeAck(Handshake),
    // SyncTx(SyncTx),
    SyncTxHash(SyncTxHash), // TxHashSyn
    RequestTxs(SyncTxHash), // TxHashAck
                            // TxSyn
                            // TxAck
}
