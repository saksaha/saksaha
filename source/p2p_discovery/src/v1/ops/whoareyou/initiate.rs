use super::{check, WHO_ARE_YOU_EXPIRATION_SEC};
use crate::{
    msg::WhoAreYou,
    state::DiscState,
    table::{Node, NodeStatus, UnknownAddrNode},
};
use p2p_identity::addr::UnknownAddr;
use std::sync::Arc;
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum WhoAreYouInitError {
    #[error("Can't send request to myself, addr: {addr}")]
    MyEndpoint { addr: UnknownAddr },

    #[error("Can't make a message (WhoAreYouAck), err: {err}")]
    MsgCreateFail { err: String },

    #[error("Can't send a message through udp socket, err: {err}")]
    MsgSendFail { err: String },

    #[error("No available addr node")]
    AddrNodeReserveFail,

    #[error(
        "Previous WhoAreYou succeeded entry still lives in the table, \
        disc_endpoint: {disc_endpoint}"
    )]
    WhoAreYouNotExpired { disc_endpoint: String },
}

pub(crate) async fn init_who_are_you(
    addr: UnknownAddr,
    disc_state: Arc<DiscState>,
) -> Result<(), WhoAreYouInitError> {
    let disc_endpoint = addr.disc_endpoint();
    let src_disc_port = disc_state.disc_port;

    if check::is_my_endpoint(src_disc_port, &addr.disc_endpoint()) {
        return Err(WhoAreYouInitError::MyEndpoint { addr });
    }

    let table = disc_state.table.clone();

    let (mut node_lock, node) =
        match table.get_mapped_node_lock(&disc_endpoint).await {
            Some(n) => n,
            None => match table.get_empty_node_lock().await {
                Some(n) => n,
                None => {
                    return Err(WhoAreYouInitError::AddrNodeReserveFail);
                }
            },
        };

    match &*node_lock {
        Node::KnownAddr(known_addr_node) => {
            if !check::is_who_are_you_expired(
                WHO_ARE_YOU_EXPIRATION_SEC,
                known_addr_node.addr.known_at,
            ) {
                return Err(WhoAreYouInitError::WhoAreYouNotExpired {
                    disc_endpoint: known_addr_node.addr.disc_endpoint(),
                });
            }
        }
        _ => {}
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

    match disc_state
        .udp_conn
        .write_msg(&disc_endpoint, way_syn_msg)
        .await
    {
        Ok(_) => {
            *node_lock = Node::UnknownAddr(UnknownAddrNode {
                addr,
                status: NodeStatus::Initialized,
            });

            table.insert_mapping(&disc_endpoint, node).await;
        }
        Err(err) => return Err(WhoAreYouInitError::MsgSendFail { err }),
    };

    Ok(())
}
