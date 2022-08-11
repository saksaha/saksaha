use sak_p2p_discovery::DiscAddr;
use sak_p2p_id::Identity;
use sak_p2p_peertable::{Peer, PeerTable};
use sak_types::{BlockHash, BlockHeight, TxCandidate, TxHash};
use std::sync::Arc;

#[derive(Debug)]
pub(in crate::node) enum NodeTask {
    SendHello {
        her_public_key: String,
    },
    SendTxHashSyn {
        tx_hashes: Vec<TxHash>,
        her_public_key: Option<String>,
    },
    SendTxSyn {
        tx_candidates: Vec<TxCandidate>,
        her_public_key: Option<String>,
    },
    SendBlockHashSyn {
        new_blocks: Vec<(BlockHeight, BlockHash)>,
        her_public_key: Option<String>,
    },
}

impl std::fmt::Display for NodeTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SendHello { .. } => {
                write!(f, "SendHello",)
            }
            Self::SendTxHashSyn { .. } => {
                write!(f, "SendTxHashSyn",)
            }
            Self::SendTxSyn { .. } => {
                write!(f, "SendTxSyn",)
            }
            Self::SendBlockHashSyn { .. } => {
                write!(f, "SendBlockHashSyn",)
            }
        }
    }
}
