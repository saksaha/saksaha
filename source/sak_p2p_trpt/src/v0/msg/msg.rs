use crate::{Handshake, TxHashSync, TxSyn};

pub enum Msg {
    HandshakeSyn(Handshake),
    HandshakeAck(Handshake),
    TxSyn(TxSyn),
    TxHashSyn(TxHashSync),
    TxHashAck(TxHashSync),
}
