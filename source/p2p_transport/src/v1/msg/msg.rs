use crate::Handshake;

pub enum Msg {
    HandshakeSyn(Handshake),
    HandshakeAck(Handshake),
}
