use crate::{Handshake, SyncMsg};

pub enum Msg {
    HandshakeSyn(Handshake),
    HandshakeAck(Handshake),
    Sync(SyncMsg),
}
