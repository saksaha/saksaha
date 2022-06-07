use super::{miner::Miner, peer_node::PeerNode};
use crate::{machine::Machine, node::msg_handler};
use futures::{stream::SplitStream, SinkExt, StreamExt};
use log::{debug, warn};
use sak_blockchain::BlockchainEvent;
use sak_p2p_ptable::{Peer, PeerStatus, PeerTable};
use sak_p2p_trpt::{Connection, Msg, SyncTx, SyncTxHash};
use std::sync::Arc;

pub(crate) struct LocalNode {
    pub(crate) peer_table: Arc<PeerTable>,
    pub(crate) machine: Arc<Machine>,
    pub(crate) miner: bool,
    pub(crate) mine_interval: Option<u64>,
}

impl LocalNode {
    pub(crate) async fn run(&self) {
        // let peer_node_rt = PeerNodeRoutine {};
        // tokio::spawn(async {
        //     peer_node_rt.run();
        // });

        println!("power");

        let machine = self.machine.clone();
        let mine_interval = self.mine_interval.clone();
        tokio::spawn(async move {
            let miner = Miner {
                machine,
                mine_interval,
            };

            miner.run().await;
        });

        let peer_it = self.peer_table.new_iter();
        let mut peer_it_lock = peer_it.write().await;

        loop {
            let peer = match peer_it_lock.next().await {
                Ok(p) => p.clone(),
                Err(_) => continue,
            };

            let peer_node = PeerNode { peer };
            let machine = self.machine.clone();

            tokio::spawn(async move {
                run_node_routine(peer_node, machine).await;
            });
        }
    }
}

async fn run_node_routine(peer_node: PeerNode, machine: Arc<Machine>) {
    loop {
        let mut conn = peer_node.peer.transport.conn.write().await;
        let mut bc_event_rx = machine.blockchain.bc_event_rx.write().await;

        tokio::select! {
            Some(ev) = bc_event_rx.recv() => {
                match ev {
                    BlockchainEvent::TxPoolStat(txs) => {
                        match conn.socket_tx
                            .send(Msg::SyncTx(SyncTx{ txs }))
                            .await {
                            Ok(_) => {
                                debug!(
                                    "Incoming tx successfully synced with \
                                    the peer node"
                                );
                            },
                            Err(err) => {
                                debug!(
                                    "Failed to sync tx with the peer nodes, \
                                    err: {}",
                                    err,
                                );
                            }
                        };
                    },
                    BlockchainEvent::TxPoolChanged(tx_hashs) => {
                        match conn.socket_tx
                            .send(Msg::SyncTxHash(SyncTxHash{ tx_hashs }))
                            .await {
                            Ok(_) => {
                                debug!(
                                    "Incoming tx_hashs successfully synced \
                                    with the peer node"
                                );
                            },
                            Err(err) => {
                                debug!(
                                    "Failed to sync tx_hashs with the peer \
                                    nodes,
                                    err: {}",
                                    err,
                                );
                            }
                        };
                    }
                }
            },
            maybe_msg = conn.socket_rx.next() => {
                match maybe_msg {
                    Some(maybe_msg) => match maybe_msg {
                        Ok(msg) => {
                            msg_handler::handle_msg(msg, &machine).await;
                        }
                        Err(err) => {
                            warn!("Failed to parse the msg, err: {}", err);
                        }
                    }
                    None => {
                        warn!("Peer has ended the connection");

                        let mut status_lock = peer_node.peer.status.write()
                            .await;

                        *status_lock = PeerStatus::Disconnected;
                        return;
                    }
                };
            }
        }
    }
}

async fn dial_temp(socket_tx: &Connection) {
    println!("awefawe");

    // let mut socket_tx_lock = peer_lock.transport.conn.socket_tx.write().await;

    // println!("send!!!");

    // match socket_tx_lock.send(Msg::Sync(SyncTx { value: 5 })).await {
    //     Ok(m) => m,
    //     Err(err) => {
    //         println!("Err; {}", err);
    //     }
    // }
}
