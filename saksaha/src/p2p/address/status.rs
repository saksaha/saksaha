#[derive(Debug, PartialEq)]
pub enum Status<C> {
    UnInitialized,

    DiscoverySucceeded,

    DiscoveryFailed(C),

    HandshakeSucceeded,

    HandshakeFailed(C),
}
