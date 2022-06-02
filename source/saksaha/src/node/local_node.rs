use super::{listener::PeerListener, peer_node::PeerNode};
use crate::{blockchain::Hash, machine::Machine};
use futures::{stream::SplitSink, SinkExt};
use futures::{stream::SplitStream, StreamExt};
use logger::tdebug;
use p2p_peer_table::{Peer, PeerTable};
use p2p_transport::{Connection, Msg, P2PCodec, SyncMsg};
use std::{sync::Arc, time::Duration};
use tokio::{net::TcpStream, sync::RwLock};
use tokio_util::codec::Framed;

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
                Ok(ref p) => {
                    println!("Hi, I am in syncing!!!!!!!!!!!!");

                    p.clone()
                }
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

    let mut interval = tokio::time::interval(Duration::from_secs(3));

    tokio::select! {
        _ = interval.tick() => {
            println!("power");
            conn;
        },
        msg = conn.socket_rx.next() => {
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
