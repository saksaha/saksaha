use crate::msg::{Msg, MsgType, WhoAreYou};
use crate::ops::whoareyou;
use crate::state::DiscState;
use crate::table::{KnownAddrNode, Node, NodeStatus};
use chrono::Utc;
use colored::Colorize;
use logger::tdebug;
use p2p_identity::addr::KnownAddr;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::Semaphore;

pub(super) struct Handler {
    pub(crate) conn_semaphore: Arc<Semaphore>,
    pub(crate) disc_state: Arc<DiscState>,
    pub(crate) socket_addr: SocketAddr,
    pub(crate) msg: Msg,
}

impl Handler {
    pub(super) async fn run(&self) -> Result<(), String> {
        match self.msg.msg_type {
            MsgType::WhoAreYouSyn => {
                let way_syn = match WhoAreYou::from_msg(&self.msg) {
                    Ok(w) => w,
                    Err(err) => {
                        return Err(format!(
                            "Error parsing who are you syn msg, err: {}",
                            err
                        ));
                    }
                };

                match whoareyou::recv_who_are_you(
                    self.socket_addr,
                    self.disc_state.clone(),
                    way_syn,
                )
                .await
                {
                    Ok(_) => (),
                    Err(err) => {
                        tdebug!(
                            "p2p_discovery",
                            "listener",
                            "WhoAreYouRecv fail, err: {}",
                            err
                        );
                    }
                };
            }
            MsgType::WhoAreYouAck => {
                let way_ack = match WhoAreYou::from_msg(&self.msg) {
                    Ok(w) => w,
                    Err(err) => {
                        return Err(format!(
                            "Error parsing who are you syn msg, err: {}",
                            err
                        ));
                    }
                };

                let public_key =
                    match crypto::convert_public_key_str_into_public_key(
                        &way_ack.src_public_key_str,
                    ) {
                        Ok(p) => p,
                        Err(err) => return Err(err),
                    };

                let known_addr = KnownAddr {
                    ip: self.socket_addr.ip().to_string(),
                    disc_port: way_ack.src_disc_port,
                    p2p_port: way_ack.src_p2p_port,
                    sig: way_ack.src_sig,
                    public_key_str: way_ack.src_public_key_str,
                    public_key,
                    known_at: Utc::now(),
                };

                let p2p_endpoint = known_addr.p2p_endpoint();
                let disc_endpoint = known_addr.disc_endpoint();

                let disc_state = self.disc_state.clone();
                let table = disc_state.table.clone();

                let (mut node_lock, node) =
                    match table.get_mapped_node_lock(&disc_endpoint).await {
                        Some(n) => n,
                        None => {
                            return Err(format!(
                                "Cannot proceed with WhoAreYouAck msg, \
                            entry does not exist in the addr table",
                            ))
                        }
                    };

                if let Node::Empty = &mut *node_lock {
                    return Err(format!(
                        "Empty node, at a point where we handle WhoAreYouAck\
                        AddrNode should exist in the table"
                    ));
                }

                *node_lock = Node::KnownAddr(KnownAddrNode {
                    addr: known_addr,
                    status: NodeStatus::WhoAreYouRecv,
                });

                drop(node_lock);

                match disc_state.table.add_known_node(node).await {
                    Ok(_) => {
                        tdebug!(
                            "p2p_discovery",
                            "server",
                            "Discovery success, her p2p endpoint: {}",
                            p2p_endpoint.green(),
                        );

                        // disc_state.table.print_all_nodes().await;
                    }
                    Err(err) => {
                        return Err(err);
                    }
                };
            }
        };

        Ok(())
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        self.conn_semaphore.add_permits(1);
    }
}
