pub mod MsgType {
    pub const HANDSHAKE_SYN: &str = "hs_syn";

    pub const HANDSHAKE_ACK: &str = "hs_ack";

    pub const TX_SYN: &str = "tx_syn";

    pub const TX_ACK: &str = "tx_ack";

    pub const TX_HASH_SYN: &str = "tx_hash_syn";

    pub const TX_HASH_ACK: &str = "tx_hash_ack";

    pub const BLOCK_HASH_SYN: &str = "block_hash_syn";

    pub const BLOCK_HASH_ACK: &str = "block_hash_ack";

    pub const BLOCK_SYN: &str = "block_syn";

    pub const BLOCK_ACK: &str = "block_ack";

    pub const PING: &str = "ping";

    pub const ERROR: &str = "error";
}
