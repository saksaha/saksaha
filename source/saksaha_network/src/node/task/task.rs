use sak_p2p_addr::UnknownAddr;
use sak_types::{BlockHash, BlockHeight, TxHash};

#[derive(Debug)]
pub(in crate::node) enum NodeTask {
    SendHelloSyn {
        unknown_addrs: Vec<UnknownAddr>,
    },
    SendTxHashSyn {
        tx_hashes: Vec<TxHash>,
    },
    SendTxSyn {
        tx_hashes: Vec<TxHash>,
    },
    SendBlockHashSyn {
        new_blocks: Vec<(BlockHeight, BlockHash)>,
    },
    SendBlockSyn {
        new_blocks: Vec<(BlockHeight, BlockHash)>,
    },
}

impl std::fmt::Display for NodeTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SendHelloSyn { .. } => {
                write!(f, "SayHello",)
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
            Self::SendBlockSyn { .. } => {
                write!(f, "SendBlockSyn",)
            }
        }
    }
}
