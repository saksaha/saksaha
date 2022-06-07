use crate::machine::Machine;
use log::warn;
use sak_logger::tinfo;
use sak_p2p_trpt::Msg;

pub(crate) async fn handle_msg(msg: Msg, machine: &Machine) {
    match msg {
        Msg::SyncTx(h) => {
            tinfo!(
                "saksaha",
                "node",
                "Discovered transactions inserted into tx pool",
            );

            machine.blockchain.insert_into_pool(h.txs).await;
        }
        Msg::SyncTxHash(h) => {
            tinfo!(
                "saksaha",
                "node",
                "Found sink request will be inserted after hash value \
                comparison.",
            );

            // machine.blockchain.compare_with_my_pool(h.tx_hashs).await;
        }
        Msg::HandshakeSyn(_) => {
            warn!("Peer has sent invalid type message, type: HandshakeSyn");
        }
        Msg::HandshakeAck(_) => {
            warn!("Peer has sent invalid type message, type: HandshakeAck");
        }
        _ => {}
    };
}
