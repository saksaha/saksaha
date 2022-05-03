use super::check;
use crate::{
    msg::WhoAreYou,
    state::DiscState,
    table::{NodeStatus, NodeValue},
};
use p2p_identity::addr::Addr;
use std::{net::SocketAddr, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum WhoAreYouRecvError {
    #[error("Can't take request I sent, addr: {addr}")]
    MyEndpoint { addr: Addr },

    #[error("Can't take a slot in the table, err: {err}")]
    TableIsFull { err: String },

    #[error("Can't make a message (WhoAreYouAck), err: {err}")]
    MsgCreateFail { err: String },

    #[error("Can't send a message through udp socket, err: {err}")]
    MsgSendFail { err: String },

    #[error("Table node is empty")]
    EmptyNode,
}

pub(crate) async fn recv_who_are_you(
    socket_addr: SocketAddr,
    disc_state: Arc<DiscState>,
    msg: WhoAreYou,
) -> Result<(), WhoAreYouRecvError> {
    let WhoAreYou {
        src_sig: her_sig,
        src_disc_port: her_disc_port,
        src_p2p_port: her_p2p_port,
        src_public_key: her_public_key,
    } = msg;

    let addr = Addr {
        ip: socket_addr.ip().to_string(),
        disc_port: her_disc_port,
        p2p_port: Some(her_p2p_port),
        sig: Some(her_sig),
        public_key: Some(her_public_key),
    };

    if check::is_my_endpoint(disc_state.disc_port, &addr) {
        return Err(WhoAreYouRecvError::MyEndpoint { addr });
    }

    let node = match disc_state.table.upsert(&addr).await {
        Ok(n) => n,
        Err(err) => return Err(WhoAreYouRecvError::TableIsFull { err }),
    };

    let mut node_lock = node.lock().await;
    let mut node_value = match &mut node_lock.value {
        NodeValue::Valued(v) => v,
        NodeValue::Empty => {
            return Err(WhoAreYouRecvError::EmptyNode);
        }
    };

    let endpoint = addr.disc_endpoint();
    let my_disc_port = disc_state.disc_port;
    let my_p2p_port = disc_state.p2p_port;
    let my_sig = disc_state.p2p_identity.sig;
    let my_public_key = disc_state.p2p_identity.public_key.clone();

    let way = WhoAreYou {
        src_sig: my_sig,
        src_disc_port: my_disc_port,
        src_p2p_port: my_p2p_port,
        src_public_key: my_public_key,
    };

    let way_msg = match way.into_ack_msg() {
        Ok(m) => m,
        Err(err) => {
            return Err(WhoAreYouRecvError::MsgCreateFail { err });
        }
    };

    match disc_state.udp_conn.write_msg(endpoint, way_msg).await {
        Ok(_) => {
            node_value.status = NodeStatus::WhoAreYouAckSent;
        }
        Err(err) => {
            return Err(WhoAreYouRecvError::MsgSendFail { err });
        }
    };

    Ok(())
}
