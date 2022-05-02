use p2p_identity::addr::Addr;

// use super::check;
// use super::msg::{WhoAreYouAck, WhoAreYouSyn};
// use crate::v1::address::Address;
// // use crate::v1::ops::Message;
// use crate::v1::DiscState;
// use logger::tdebug;
// use std::sync::Arc;
use crate::{
    msg::{self, WhoAreYou},
    state::DiscState,
    table::{Node, NodeStatus, NodeValue, NodeValueInner},
};
use std::{net::SocketAddr, sync::Arc};
use thiserror::Error;

use super::check;

#[derive(Error, Debug)]
pub(crate) enum WhoAreYouRecvError {
    // #[error("Cannot convert to byte, _err: {err}")]
    // ByteConversionFail { err: String },
    #[error("Can't take request I sent, addr: {addr}")]
    MyEndpoint { addr: Addr },

    #[error("Can't take a slot in the table, addr: {addr}")]
    TableIsFull { addr: Addr },

    #[error("Can't make a message (WhoAreYouAck), err: {err}")]
    MsgCreateFail { err: String },

    #[error("Can't send a message through udp socket, err: {err}")]
    MsgSendFail { err: String },

    #[error("Table node is empty")]
    EmptyNode,
    // #[error("Couldn't parse WhoAreYou message, err: {err}")]
    // MessageParseFail { err: String },

    // #[error(
    //     "Couldn't reserve node, table is full, endpoint:\
    //     {endpoint}, err: {err}"
    // )]
    // TableIsFull { endpoint: String, err: String },

    // #[error("Couldn't sent msg through socket")]
    // SendFail {
    //     #[from]
    //     source: std::io::Error,
    // },

    // #[error("Can't add node to table, err: {err}")]
    // TableAddFail { err: String },
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
        Err(err) => return Err(WhoAreYouRecvError::TableIsFull { addr }),
    };

    let mut node_lock = node.lock().await;
    let mut node_value = match &mut node_lock.value {
        NodeValue::Valued(v) => v,
        NodeValue::Empty => {
            return Err(WhoAreYouRecvError::EmptyNode);
        }
    };
    // node_lock.value = NodeValue::Valued(NodeValueInner { addr, status: });

    // let table = disc_state.table.clone();
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

    let way_msg = match way.into_msg() {
        Ok(m) => m,
        Err(err) => {
            return Err(WhoAreYouRecvError::MsgCreateFail { err });
        }
    };

    match disc_state.udp_conn.write_msg(endpoint, way_msg).await {
        Ok(_) => {
            node_value.status = NodeStatus::WhoAreYouSynSent;
        }
        Err(err) => {
            return Err(WhoAreYouRecvError::MsgSendFail { err });
        }
    };

    // let way_syn = match WhoAreYouSyn::parse(buf) {
    //     Ok(m) => m,
    //     Err(err) => {
    //         return Err(WhoareyouRecvError::MessageParseFail { err });
    //     }
    // };

    // match disc_state
    //     .table
    //     .add(table_node, |mut val| {
    //         *val = NodeValue::new_identified(
    //             addr.clone(),
    //             way_syn.way.sig,
    //             way_syn.way.p2p_port,
    //             way_syn.way.public_key_bytes,
    //         );
    //         val
    //     })
    //     .await
    // {
    //     Ok((.., endpoint)) => {
    //         tdebug!(
    //             "p2p_discovery",
    //             "whoareyou",
    //             "Node is discovered, I received, endpoint: {}",
    //             endpoint
    //         );
    //     }
    //     Err(err) => return Err(WhoareyouRecvError::TableAddFail { err }),
    // };

    // let _send_who_are_you_ack = {
    //     let disc_port = disc_state.disc_port;
    //     let p2p_port = disc_state.p2p_port;

    //     let endpoint = addr.disc_endpoint();

    //     if check::is_my_endpoint(disc_port, &endpoint) {
    //         return Err(WhoareyouRecvError::MyEndpoint { endpoint });
    //     }

    //     let sig = disc_state.p2p_identity.sig;

    //     let way_ack = WhoAreYouAck::new(
    //         sig,
    //         p2p_port,
    //         disc_state.p2p_identity.public_key,
    //     );

    //     let buf = match way_ack.to_bytes() {
    //         Ok(b) => b,
    //         Err(err) => {
    //             return Err(WhoareyouRecvError::ByteConversionFail { err });
    //         }
    //     };

    //     disc_state
    //         .udp_socket
    //         .send_to(&buf, endpoint.clone())
    //         .await?;

    //     tdebug!(
    //         "p2p_discovery",
    //         "whoareyou",
    //         "Successfully sent WhoAreYouAck to endpoint: {}, len: {}",
    //         &endpoint,
    //         buf.len(),
    //     );
    // };

    Ok(())
}
