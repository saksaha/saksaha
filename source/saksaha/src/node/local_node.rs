use super::{listener::PeerListener, peer_node::PeerNode};
use crate::machine::Machine;
use blockchain::BlockchainEvent;
use futures::{stream::SplitStream, SinkExt, StreamExt};
use p2p_peer_table::{Peer, PeerTable};
use p2p_transport::{Connection, Msg, P2PCodec, SyncMsg};
use std::sync::Arc;

pub(crate) struct LocalNode {
    pub(crate) peer_table: Arc<PeerTable>,
    pub(crate) machine: Arc<Machine>,
}

impl LocalNode {
    pub(crate) async fn run(&self) {
        let it = self.peer_table.new_iter();
        let mut it_lock = it.write().await;

        loop {
            let peer = match it_lock.next().await {
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
