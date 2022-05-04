use super::check;
use crate::{
    msg::WhoAreYou,
    state::DiscState,
    table::{NodeStatus, NodeValue},
};
use p2p_identity::addr::Addr;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum WhoAreYouInitError {
    #[error("Can't send request to myself, addr: {addr}")]
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

pub(crate) async fn init_who_are_you(
    addr: Addr,
    disc_state: Arc<DiscState>,
) -> Result<(), WhoAreYouInitError> {
    let endpoint = addr.disc_endpoint();
    let src_disc_port = disc_state.disc_port;

    if check::is_my_endpoint(src_disc_port, &addr) {
        return Err(WhoAreYouInitError::MyEndpoint { addr });
    }

    let table = disc_state.table.clone();

    let node = match table.upsert(&addr, NodeStatus::Initialized).await {
        Ok(a) => a,
        Err(err) => {
            return Err(WhoAreYouInitError::TableIsFull { err });
        }
    };

    let mut node_lock = node.lock().await;
    let mut node_value = match &mut node_lock.value {
        NodeValue::Valued(v) => v,
        _ => return Err(WhoAreYouInitError::EmptyNode),
    };

    let src_disc_port = disc_state.disc_port;
    let src_p2p_port = disc_state.p2p_port;
    let src_sig = disc_state.p2p_identity.sig;
    let src_public_key_str = disc_state.p2p_identity.public_key_str.clone();

    let way = WhoAreYou {
        src_sig,
        src_disc_port,
        src_p2p_port,
        src_public_key_str,
    };

    let way_syn_msg = match way.into_syn_msg() {
        Ok(m) => m,
        Err(err) => return Err(WhoAreYouInitError::MsgCreateFail { err }),
    };

    match disc_state.udp_conn.write_msg(endpoint, way_syn_msg).await {
        Ok(_) => {
            node_value.status = NodeStatus::WhoAreYouSynSent;
        }
        Err(err) => return Err(WhoAreYouInitError::MsgSendFail { err }),
    };

    Ok(())
}
