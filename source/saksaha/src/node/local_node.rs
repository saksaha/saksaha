use super::{listener::PeerListener, miner::Miner, peer_node::PeerNode};
use crate::machine::Machine;
use futures::{stream::SplitStream, SinkExt, StreamExt};
use sak_blockchain::BlockchainEvent;
use sak_p2p_ptable::{Peer, PeerTable};
use sak_p2p_trpt::{Connection, Msg, P2PCodec, SyncMsg};
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
                Ok(p) => p,
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
    let peer = peer_node.peer.clone();
    let mut conn = peer.transport.conn.write().await;

    let mut bc_event_rx = machine.blockchain.bc_event_rx.write().await;

    tokio::select! {
        Some(ev) = bc_event_rx.recv() => {
            match ev {
                BlockchainEvent::TxPoolChange(tx_hash) => {
                    println!("got hash from my rpc, {:?}", tx_hash);

                    match conn.socket_tx.send(Msg::Sync(SyncMsg{value:1})).await {
                        Ok(_) => println!("successfully send a msg to other nodes"),
                        Err(err) => println!("failed to send a msg to other nodes, err: {}", err),
                    };
                },
            }
        },
        msg = conn.socket_rx.next() => {
            println!("got hash from other node");
            conn;
        }
    }
}

async fn dial_temp(socket_tx: &Connection) {
    println!("awefawe");

    // let mut socket_tx_lock = peer_lock.transport.conn.socket_tx.write().await;

    // println!("send!!!");

    // match socket_tx_lock.send(Msg::Sync(SyncMsg { value: 5 })).await {
    //     Ok(m) => m,
    //     Err(err) => {
    //         println!("Err; {}", err);
    //     }
    // }
}
