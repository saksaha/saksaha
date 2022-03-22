pub struct Identity {
    pub secret: String,
    pub public_key: String,
}

pub struct P2PConfig {
    pub local_1: Identity,
    pub local_2: Identity,
}

pub struct DevLocal {
    p2p: P2PConfig,
}

pub fn get_dev_local_config() -> DevLocal {
    return DevLocal {
        p2p: P2PConfig {
            local_1: Identity {
                secret: String::from("
                    9a3d9fafda4a1b0e420d339a3e42c2c81062ba788018eb6639b5d2eedeb2d13e
                "),
                public_key: String::from("
                04715796a40b0d58fc14a3c4ebee21cb806763066a7f1a17adbc256999764443beb8109cfd000718535c5aa27513a2edafc6e8bdbe7c27edc2980f9bbc25142fc5
                "),
            },
            local_2: Identity {
                secret: "".to_string(),
                public_key: "".to_string(),
            },
        },
    };
}
