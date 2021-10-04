#[derive(Debug, PartialEq)]
pub enum Status<C> {
    NotInitialized,

    DiscoverySucceeded,

    DiscoveryFailed(C),

    HandshakeSucceeded,

    HandshakeFailed(C),
}
