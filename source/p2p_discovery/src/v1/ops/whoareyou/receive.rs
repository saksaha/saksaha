use super::check;
use crate::{
    msg::WhoAreYou,
    state::DiscState,
    table::{NodeStatus, NodeValue},
};
use chrono::Utc;
use colored::Colorize;
use logger::{tdebug, terr};
use p2p_identity::addr::{Addr, KnownAddr};
use std::{net::SocketAddr, sync::Arc, time::SystemTime};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum WhoAreYouRecvError {
    #[error("Can't take request I sent, addr: {addr}")]
    MyEndpoint { addr: KnownAddr },

    #[error("Can't take a slot in the table, err: {err}")]
    TableIsFull { err: String },

    #[error("Can't make a message (WhoAreYouAck), err: {err}")]
    MsgCreateFail { err: String },

    #[error("Can't send a message through udp socket, err: {err}")]
    MsgSendFail { err: String },

    #[error("Table node is empty")]
    EmptyNode,

    #[error("Could not register as a known node, endpoint: {endpoint}")]
    KnownNodeRegisterFail { endpoint: String, err: String },

    #[error("Could not make public key out of string: {public_key_str}")]
    PublicKeyCreateFail { public_key_str: String, err: String },
}

pub(crate) async fn recv_who_are_you(
    socket_addr: SocketAddr,
    disc_state: Arc<DiscState>,
    way_recv: WhoAreYou,
) -> Result<(), WhoAreYouRecvError> {
    let WhoAreYou {
        src_sig: her_sig,
        src_disc_port: her_disc_port,
        src_p2p_port: her_p2p_port,
        src_public_key_str: her_public_key_str,
    } = way_recv;

    let her_public_key = match crypto::convert_public_key_str_into_public_key(
        &her_public_key_str,
    ) {
        Ok(p) => p,
        Err(err) => {
            return Err(WhoAreYouRecvError::PublicKeyCreateFail {
                public_key_str: her_public_key_str,
                err: err.to_string(),
            });
        }
    };

    let addr = KnownAddr {
        ip: socket_addr.ip().to_string(),
        disc_port: her_disc_port,
        p2p_port: her_p2p_port,
        sig: her_sig,
        public_key_str: her_public_key_str,
        public_key: her_public_key,
        known_at: Utc::now(),
    };

    let known_at = addr.known_at;
    let endpoint = addr.disc_endpoint();
    let her_p2p_endpoint = addr.p2p_endpoint();

    if check::is_my_endpoint(disc_state.disc_port, &endpoint) {
        return Err(WhoAreYouRecvError::MyEndpoint { addr });
    }

    let node = match disc_state
        .table
        .upsert(
            Addr::Known(addr),
            NodeStatus::WhoAreYouRecv { fail_count: 0 },
        )
        .await
    {
        Ok(n) => n,
        Err(err) => return Err(WhoAreYouRecvError::TableIsFull { err }),
    };

    let mut node_lock = node.lock().await;
    let mut _node_value = match &mut node_lock.value {
        NodeValue::Valued(v) => v,
        _ => {
            return Err(WhoAreYouRecvError::EmptyNode);
        }
    };

    let my_disc_port = disc_state.disc_port;
    let my_p2p_port = disc_state.p2p_port;
    let my_sig = disc_state.p2p_identity.sig;
    let my_public_key_str = disc_state.p2p_identity.public_key_str.clone();

    let way = WhoAreYou {
        src_sig: my_sig,
        src_disc_port: my_disc_port,
        src_p2p_port: my_p2p_port,
        src_public_key_str: my_public_key_str,
    };

    let way_msg = match way.into_ack_msg() {
        Ok(m) => m,
        Err(err) => {
            return Err(WhoAreYouRecvError::MsgCreateFail { err });
        }
    };

    match disc_state
        .udp_conn
        .write_msg(endpoint.clone(), way_msg)
        .await
    {
        Ok(_) => {
            drop(node_lock);
            match disc_state.table.add_known_node(node).await {
                Ok(_) => {
                    tdebug!(
                        "p2p_discovery",
                        "whoareyou",
                        "Discovery success, her p2p_endpoint: {}, known_at: {}",
                        her_p2p_endpoint.green(),
                        known_at,
                    );
                }
                Err(err) => {
                    terr!(
                        "p2p_discovery",
                        "whoareyou",
                        "Fail to add known node. Queue might have been closed",
                    );

                    return Err(WhoAreYouRecvError::KnownNodeRegisterFail {
                        endpoint,
                        err,
                    });
                }
            };
        }
        Err(err) => {
            return Err(WhoAreYouRecvError::MsgSendFail { err });
        }
    };

    Ok(())
}
