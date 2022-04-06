pub mod dev_local;

pub struct Identity {
    pub secret: String,
    pub public_key: String,
}

pub struct P2PConfig {
    pub local_1: Identity,
    pub local_2: Identity,
    pub local_3: Identity,
    pub local_4: Identity,
}
