use crate::{Handshake, TxHashSync, TxSyn};

#[derive(Debug)]
pub enum Msg {
    HandshakeSyn(Handshake),
    HandshakeAck(Handshake),
    TxSyn(TxSyn),
    TxHashSyn(TxHashSync),
    TxHashAck(TxHashSync),
}
