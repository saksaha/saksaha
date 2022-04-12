pub(crate) mod default;

pub(crate) struct Config {
    p2p: P2PConfig,
}

pub(crate) struct Identity {
    pub secret: String,
    pub public_key: String,
}

pub(crate) struct P2PConfig {
    pub(crate) identity: Identity,
    pub(crate) peers: Vec<Identity>,
}
