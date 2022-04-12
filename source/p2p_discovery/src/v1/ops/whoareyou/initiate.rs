use super::check;
use super::msg::{WhoAreYouAck, WhoAreYouSyn};
use crate::v1::address::Address;
use crate::v1::ops::Message;
use crate::v1::table::{Node, NodeValue};
use crate::v1::DiscState;
use log::debug;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::UdpSocket;
use tokio::sync::mpsc::error::TrySendError;

#[derive(Error, Debug)]
pub enum WhoareyouInitError {
    #[error("Request to myself, dumping this task, endpoint: {endpoint}")]
    MyEndpoint { endpoint: String },

    #[error(" Cannot reserve Node, _err: {err}")]
    NodeReserveFail { err: String },

    #[error(" Couldn't sent msg through socket")]
    SendFail(#[from] std::io::Error),

    #[error(" Cannot convert to byte, _err: {err}")]
    ByteConversionFail { err: String },

    #[error(" Cannot create verifying key of remote, _err: {err}")]
    VerifiyingKeyFail { err: String },

    #[error(" Signature is invalid, buf: {buf:?}, _err: {err}")]
    InvalidSignature { buf: Vec<u8>, err: String },

    #[error(
        " Failed to register node into map, \
        endpoint: {endpoint}, _err: {err}"
    )]
    NodeRegisterFail { endpoint: String, err: String },

    #[error(" Can't parse WhoAreYou message, err: {err}")]
    MessageParseFail { err: String },

    #[error(
        " Can't reserve node, table is full, \
        endpoint: {endpoint}, err: {err}"
    )]
    TableIsFull { endpoint: String, err: String },

    #[error(" Can't add node to table, err: {err}")]
    TableAddFail { err: String },

    #[error(" Can't put back a node to table")]
    NodePutBackFail {
        #[from]
        source: TrySendError<Arc<Node>>,
    },
}

struct Initiate;

impl Initiate {}

pub(crate) async fn send_who_are_you(
    disc_state: Arc<DiscState>,
    addr: Address,
) -> Result<(), WhoareyouInitError> {
    let my_disc_port = disc_state.my_disc_port;
    let my_p2p_port = disc_state.my_p2p_port;

    let endpoint = addr.disc_endpoint();

    if check::is_my_endpoint(my_disc_port, &endpoint) {
        return Err(WhoareyouInitError::MyEndpoint { endpoint });
    }

    let my_sig = disc_state.identity.sig;
    let my_public_key = disc_state.identity.public_key;

    let way_syn = WhoAreYouSyn::new(my_sig, my_p2p_port, my_public_key);

    let buf = match way_syn.to_bytes() {
        Ok(b) => b,
        Err(err) => {
            return Err(WhoareyouInitError::ByteConversionFail { err });
        }
    };

    disc_state
        .udp_socket
        .send_to(&buf, endpoint.clone())
        .await?;

    debug!(
        "Successfully sent WhoAreYou to endpoint: {}, buf len: {}",
        &endpoint,
        buf.len()
    );

    Ok(())
}

pub(crate) async fn handle_who_are_you_ack(
    disc_state: Arc<DiscState>,
    addr: Address,
    buf: &[u8],
) -> Result<(), WhoareyouInitError> {
    let endpoint = addr.disc_endpoint();

    let table_node = match disc_state.table.try_reserve().await {
        Ok(n) => n,
        Err(err) => {
            return Err(WhoareyouInitError::TableIsFull { endpoint, err })
        }
    };

    let way_ack = match WhoAreYouAck::parse(buf) {
        Ok(m) => m,
        Err(err) => {
            match disc_state.table.put_back(table_node) {
                Ok(_) => (),
                Err(err) => {
                    return Err(WhoareyouInitError::NodePutBackFail {
                        source: err,
                    })
                }
            };

            return Err(WhoareyouInitError::MessageParseFail { err });
        }
    };

    match disc_state
        .table
        .add(table_node, |mut val| {
            *val = NodeValue::new_identified(
                addr.clone(),
                way_ack.way.sig,
                way_ack.way.p2p_port,
                way_ack.way.public_key_bytes,
            );
            val
        })
        .await
    {
        Ok((.., endpoint)) => {
            debug!("Discovered a node, I initiated, endpoint: {}", endpoint);
        }
        Err(err) => return Err(WhoareyouInitError::TableAddFail { err }),
    };

    Ok(())
}
