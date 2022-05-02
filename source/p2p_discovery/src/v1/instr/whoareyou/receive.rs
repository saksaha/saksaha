// use super::check;
// use super::msg::{WhoAreYouAck, WhoAreYouSyn};
// use crate::v1::address::Address;
// // use crate::v1::ops::Message;
// use crate::v1::DiscState;
// use logger::tdebug;
// use std::sync::Arc;
// use thiserror::Error;
use crate::{msg::WhoAreYouSyn, state::DiscState};
use std::sync::Arc;

// #[derive(Error, Debug)]
// pub enum WhoareyouRecvError {
//     #[error("Cannot convert to byte, _err: {err}")]
//     ByteConversionFail { err: String },

//     #[error("Can't send ack to my endpoint, endpoint: {endpoint}")]
//     MyEndpoint { endpoint: String },

//     #[error("Couldn't parse WhoAreYou message, err: {err}")]
//     MessageParseFail { err: String },

//     #[error(
//         "Couldn't reserve node, table is full, endpoint:\
//         {endpoint}, err: {err}"
//     )]
//     TableIsFull { endpoint: String, err: String },

//     #[error("Couldn't sent msg through socket")]
//     SendFail {
//         #[from]
//         source: std::io::Error,
//     },

//     #[error("Can't add node to table, err: {err}")]
//     TableAddFail { err: String },
// }

pub(crate) async fn recv_who_are_you(
    disc_state: Arc<DiscState>,
    msg: WhoAreYouSyn,
) -> Result<(), String> {
    let src_disc_port = msg.src_disc_port;
    let src_p2p_port = msg.src_p2p_port;
    let src_sig = msg.src_sig;

    // let table_node = match disc_state.table.try_reserve().await {
    //     Ok(n) => n,
    //     Err(err) => {
    //         return Err(WhoareyouRecvError::TableIsFull { endpoint, err })
    //     }
    // };

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
