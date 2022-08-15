use sak_p2p_discovery::DiscAddr;
use sak_p2p_id::Identity;
use sak_p2p_peertable::{Peer, PeerTable};
use sak_types::{BlockHash, BlockHeight, TxCandidate, TxHash};
use std::sync::Arc;

#[derive(Debug)]
pub(in crate::node) enum NodeTask {
    SendTxHashSyn {
        tx_hashes: Vec<TxHash>,
    },
    SendTxSyn {
        tx_hashes: Vec<TxHash>,
        // tx_candidates: Vec<TxCandidate>,
    },
    SendBlockHashSyn {
        new_blocks: Vec<(BlockHeight, BlockHash)>,
    },
    SendBlockSyn {},
}

impl std::fmt::Display for NodeTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
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
