use super::sync::TxHashSyn;
use crate::{Handshake, TxSyn};

pub enum Msg {
    HandshakeSyn(Handshake),
    HandshakeAck(Handshake),
    TxSyn(TxSyn),
    TxHashSyn(TxHashSyn), // TxHashSyn
    TxHashAck(TxHashSyn), // TxHashAck
                          // TxSyn
                          // TxAck
}
