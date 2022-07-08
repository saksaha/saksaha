use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Tx {
    //
    created_at: String,

    //
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,

    //
    pi: Vec<u8>,

    //
    author_sig: String,

    //
    ctr_addr: String,

    //
    cm: String,
    v: String,
    k: String,
    s: String,
    sn_1: String,
    sn_2: String,
    cm_1: String,
    cm_2: String,
    rt: String,

    // auto-generated value
    tx_height: u128,
    hash: String,
}

pub struct ContractCallData {
    pub fn_name: String,
    pub args: Vec<Vec<u8>>,
}

pub enum TxType {
    ContractCall,
    ContractDeploy,
    Plain,
}

impl Tx {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        pi: Vec<u8>,
        ctr_addr: String,
        hash: String,
        cm: String,
        v: String,
        k: String,
        s: String,
        sn_1: String,
        sn_2: String,
        cm_1: String,
        cm_2: String,
        rt: String,
        tx_height: u128,
    ) -> Tx {
        Tx {
            created_at,
            data,
            pi,
            author_sig,
            ctr_addr,
            cm,
            v,
            k,
            s,
            sn_1,
            sn_2,
            cm_1,
            cm_2,
            rt,
            tx_height,
            hash,
        }
    }

    pub fn get_created_at(&self) -> &String {
        &self.created_at
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_pi(&self) -> &Vec<u8> {
        &self.pi
    }

    pub fn get_author_sig(&self) -> &String {
        &self.author_sig
    }

    pub fn get_ctr_addr(&self) -> &String {
        &self.ctr_addr
    }

    pub fn get_tx_height(&self) -> &u128 {
        &self.tx_height
    }

    pub fn get_cm(&self) -> &String {
        &self.cm
    }

    pub fn get_v(&self) -> &String {
        &self.v
    }

    pub fn get_k(&self) -> &String {
        &self.k
    }

    pub fn get_s(&self) -> &String {
        &self.s
    }

    pub fn get_sn_1(&self) -> &String {
        &self.sn_1
    }

    pub fn get_sn_2(&self) -> &String {
        &self.sn_2
    }

    pub fn get_cm_1(&self) -> &String {
        &self.cm_1
    }

    pub fn get_cm_2(&self) -> &String {
        &self.cm_2
    }

    pub fn get_rt(&self) -> &String {
        &self.rt
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }

    pub fn is_mutating_ctr_state(&self) -> bool {
        self.ctr_addr.len() > 0
    }

    pub fn has_ctr_addr(&self) -> bool {
        self.ctr_addr.len() > 0
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TxCandidate {
    created_at: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    pi: Vec<u8>,
    author_sig: String,
    ctr_addr: String,
    cm: String,

    v: String,
    k: String,
    s: String,
    sn_1: String,
    sn_2: String,
    cm_1: String,
    cm_2: String,
    rt: String,

    // auto-generated value
    hash: String,
}

impl TxCandidate {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        pi: Vec<u8>,
        ctr_addr: Option<String>,
        cm: Option<String>,
        v: Option<String>,
        k: Option<String>,
        s: Option<String>,
        sn_1: Option<String>,
        sn_2: Option<String>,
        cm_1: Option<String>,
        cm_2: Option<String>,
        rt: Option<String>,
    ) -> TxCandidate {
        let ctr_addr = ctr_addr.unwrap_or(String::from(""));
        let cm = cm.unwrap_or(String::from(""));
        let v = v.unwrap_or(String::from(""));
        let k = k.unwrap_or(String::from(""));
        let s = s.unwrap_or(String::from(""));
        let sn_1 = sn_1.unwrap_or(String::from(""));
        let sn_2 = sn_2.unwrap_or(String::from(""));
        let cm_1 = cm_1.unwrap_or(String::from(""));
        let cm_2 = cm_2.unwrap_or(String::from(""));
        let rt = rt.unwrap_or(String::from(""));

        let hashable_items = vec![
            created_at.as_bytes(),
            data.as_slice(),
            pi.as_slice(),
            author_sig.as_bytes(),
            cm.as_bytes(),
        ];

        let hash = sak_crypto::compute_hash(&hashable_items);

        TxCandidate {
            created_at,
            data,
            pi,
            author_sig,
            ctr_addr,
            cm,
            v,
            k,
            s,
            sn_1,
            sn_2,
            cm_1,
            cm_2,
            rt,
            hash,
        }
    }

    pub fn upgrade(self, tx_height: u128) -> Tx {
        Tx::new(
            self.created_at,
            self.data,
            self.author_sig,
            self.pi,
            self.ctr_addr,
            self.hash,
            self.cm,
            self.v,
            self.k,
            self.s,
            self.sn_1,
            self.sn_2,
            self.cm_1,
            self.cm_2,
            self.rt,
            tx_height,
        )
    }

    pub fn get_created_at(&self) -> &String {
        &self.created_at
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_pi(&self) -> &Vec<u8> {
        &self.pi
    }

    pub fn get_author_sig(&self) -> &String {
        &self.author_sig
    }

    pub fn get_ctr_addr(&self) -> &String {
        &self.ctr_addr
    }

    pub fn get_cm(&self) -> &String {
        &self.cm
    }

    pub fn get_v(&self) -> &String {
        &self.v
    }

    pub fn get_k(&self) -> &String {
        &self.k
    }

    pub fn get_s(&self) -> &String {
        &self.s
    }

    pub fn get_sn_1(&self) -> &String {
        &self.sn_1
    }

    pub fn get_sn_2(&self) -> &String {
        &self.sn_2
    }

    pub fn get_cm_1(&self) -> &String {
        &self.cm_1
    }

    pub fn get_cm_2(&self) -> &String {
        &self.cm_2
    }

    pub fn get_rt(&self) -> &String {
        &self.rt
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }

    pub fn is_mutating_ctr_state(&self) -> bool {
        self.ctr_addr.len() > 0
    }

    pub fn has_ctr_addr(&self) -> bool {
        self.ctr_addr.len() > 0
    }
}
