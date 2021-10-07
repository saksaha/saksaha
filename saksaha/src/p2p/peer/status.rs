#[derive(Debug, PartialEq)]
pub enum Status<E> {
    Empty,

    NotInitialized,

    DiscoverySuccess,

    HandshakeSuccess,

    HandshakeFail(E),
}
