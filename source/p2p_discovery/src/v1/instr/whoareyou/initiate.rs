// use super::check;
// use super::msg::{WhoAreYouAck, WhoAreYouSyn};
// use crate::v1::address::Address;
// // use crate::v1::ops::Message;
// use crate::v1::DiscState;
// use logger::tdebug;
// use p2p_identity::peer::UnknownPeer;
// use std::sync::Arc;
// use thiserror::Error;
// use tokio::sync::mpsc::error::TrySendError;

use p2p_identity::addr::Addr;

use crate::{
    msg::{self, Msg},
    state::DiscState,
};
use std::sync::Arc;

// #[derive(Error, Debug)]
// pub enum WhoareyouInitError {
//     #[error("Request to myself, endpoint: {endpoint}")]
//     MyEndpoint { endpoint: String },

//     #[error(" Cannot reserve Node, _err: {err}")]
//     NodeReserveFail { err: String },

//     #[error(" Couldn't sent msg through socket")]
//     SendFail(#[from] std::io::Error),

//     #[error(" Cannot convert to byte, _err: {err}")]
//     ByteConversionFail { err: String },

//     #[error(" Cannot create verifying key of remote, _err: {err}")]
//     VerifiyingKeyFail { err: String },

//     #[error(" Signature is invalid, buf: {buf:?}, _err: {err}")]
//     InvalidSignature { buf: Vec<u8>, err: String },

//     #[error(
//         " Failed to register node into map, \
//         endpoint: {endpoint}, _err: {err}"
//     )]
//     NodeRegisterFail { endpoint: String, err: String },

//     #[error(" Can't parse WhoAreYou message, err: {err}")]
//     MessageParseFail { err: String },

//     #[error(
//         " Can't reserve node, table is full, \
//         endpoint: {endpoint}, err: {err}"
//     )]
//     TableIsFull { endpoint: String, err: String },

//     #[error(" Can't add node to table, err: {err}")]
//     TableAddFail { err: String },
//     // #[error(" Can't put back a node to table")]
//     // NodePutBackFail {
//     //     #[from]
//     //     source: TrySendError<Arc<Node>>,
//     // },
// }

pub(crate) async fn init_who_are_you(
    addr: Addr,
    disc_state: Arc<DiscState>,
) -> Result<(), String> {
    let my_disc_port = disc_state.disc_port;
    let my_p2p_port = disc_state.p2p_port;
    let my_sig = disc_state.p2p_identity.sig;
    let my_public_key = disc_state.p2p_identity.public_key.clone();

    let endpoint = addr.disc_endpoint();

    let way_syn = msg::WhoAreYouSyn {
        my_sig,
        my_disc_port,
        my_p2p_port,
    };

    disc_state
        .udp_conn
        .write_msg(endpoint, way_syn.into_msg()?)
        .await;

    // let frame = match way_syn.into_frame() {
    //     Ok(f) => f,
    //     Err(err) => {
    //         return Err(format!(
    //             "Error converting WhoAreYouSyn message into a frame"
    //         ))
    //     }
    // };

    // disc_state.udp_socket.send_to(&buf, endpoint.clone()).await;

    // let way_syn = WhoAreYouSyn::new(sig, p2p_port, public_key);

    // let buf = match way_syn.to_bytes() {
    //     Ok(b) => b,
    //     Err(err) => {
    //         return Err(WhoareyouInitError::ByteConversionFail { err });
    //     }
    // };

    // disc_state
    //     .udp_socket
    //     .send_to(&buf, her_endpoint.clone())
    //     .await?;

    // tdebug!(
    //     "p2p_discovery",
    //     "whoareyou",
    //     "Successfully sent WhoAreYou to endpoint: {}, buf len: {}",
    //     &her_endpoint,
    //     buf.len()
    // );

    Ok(())
}

// pub(crate) async fn handle_who_are_you_ack(
//     disc_state: Arc<DiscState>,
//     addr: Address,
//     buf: &[u8],
// ) -> Result<(), WhoareyouInitError> {
//     let endpoint = addr.disc_endpoint();

//     // let table_node = match disc_state.table.try_reserve().await {
//     //     Ok(n) => n,
//     //     Err(err) => {
//     //         return Err(WhoareyouInitError::TableIsFull { endpoint, err })
//     //     }
//     // };

//     // let way_ack = match WhoAreYouAck::parse(buf) {
//     //     Ok(m) => m,
//     //     Err(err) => {
//     //         match disc_state.table.put_back(table_node) {
//     //             Ok(_) => (),
//     //             Err(err) => {
//     //                 return Err(WhoareyouInitError::NodePutBackFail {
//     //                     source: err,
//     //                 })
//     //             }
//     //         };

//     //         return Err(WhoareyouInitError::MessageParseFail { err });
//     //     }
//     // };

//     // match disc_state
//     //     .table
//     //     .add(table_node, |mut val| {
//     //         *val = NodeValue::new_identified(
//     //             addr.clone(),
//     //             way_ack.way.sig,
//     //             way_ack.way.p2p_port,
//     //             way_ack.way.public_key_bytes,
//     //         );
//     //         val
//     //     })
//     //     .await
//     // {
//     //     Ok((.., endpoint)) => {
//     //         tdebug!(
//     //             "p2p_discvoery",
//     //             "whoareyou",
//     //             "Discovered a node, I initiated, endpoint: {}",
//     //             endpoint
//     //         );
//     //     }
//     //     Err(err) => return Err(WhoareyouInitError::TableAddFail { err }),
//     // };

//     Ok(())
// }
