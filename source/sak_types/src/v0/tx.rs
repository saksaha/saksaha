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
    #[serde(with = "serde_bytes")]
    ctr_addr: Vec<u8>,

    // auto-generated value
    hash: String,
}

pub enum TxType {
    ContractCall,
    ContractDeploy,
    Others,
}

impl Tx {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        pi: Vec<u8>,
        ctr_addr: Option<Vec<u8>>,
    ) -> Tx {
        let ctr_addr = match ctr_addr {
            Some(a) => a,
            None => vec![],
        };

        let hash = sak_crypto::compute_hash(&[
            created_at.as_bytes(),
            data.as_slice(),
            pi.as_slice(),
            author_sig.as_bytes(),
            ctr_addr.as_slice(),
        ]);

        Tx {
            created_at,
            data,
            pi,
            author_sig,
            ctr_addr,
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

    pub fn get_ctr_addr(&self) -> &Vec<u8> {
        &self.ctr_addr
    }

    pub fn get_hash(&self) -> &String {
        &self.hash
    }
}
