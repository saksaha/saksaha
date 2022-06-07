use crate::{Handshake, SyncTx};

use super::sync::SyncTxHash;

pub enum Msg {
    HandshakeSyn(Handshake),
    HandshakeAck(Handshake),
    SyncTx(SyncTx),
    SyncTxHash(SyncTxHash),
}
