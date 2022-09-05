use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SendPourTxRequest {
    pub created_at: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    pub author_sig: String,
    pub ctr_addr: Option<String>,
    #[serde(with = "serde_bytes")]
    pub pi: Vec<u8>,
    pub sns: Vec<[u8; 32]>,
    pub cms: Vec<[u8; 32]>,
    pub merkle_rt: [u8; 32],
}

impl SendPourTxRequest {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        ctr_addr: Option<String>,
        pi: Vec<u8>,
        sns: Vec<[u8; 32]>,
        cms: Vec<[u8; 32]>,
        merkle_rt: [u8; 32],
    ) -> SendPourTxRequest {
        SendPourTxRequest {
            created_at,
            data,
            author_sig,
            ctr_addr,
            pi,
            sns,
            cms,
            merkle_rt,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMintTxRequest {
    pub created_at: String,
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,
    pub author_sig: String,
    pub ctr_addr: Option<String>,
    pub cms: Vec<[u8; 32]>,
    pub v: [u8; 32],
    pub k: [u8; 32],
    pub s: [u8; 32],
}

impl SendMintTxRequest {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        ctr_addr: Option<String>,
        cms: Vec<[u8; 32]>,
        v: [u8; 32],
        k: [u8; 32],
        s: [u8; 32],
    ) -> SendMintTxRequest {
        SendMintTxRequest {
            created_at,
            data,
            author_sig,
            ctr_addr,
            cms,
            v,
            k,
            s,
        }
    }
}
