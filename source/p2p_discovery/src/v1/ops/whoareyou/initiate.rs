use super::{check, WhoAreYou};
use crate::{
    v1::{net::Connection, ops::Msg},
    Table,
};
use futures::SinkExt;
use logger::tdebug;
use p2p_addr::UnknownAddr;
use p2p_identity::Identity;
use std::{net::SocketAddr, sync::Arc};
use thiserror::Error;

#[derive(Error, Debug)]
pub(crate) enum WhoAreYouInitError {
    #[error("Can't send request to myself, addr: {addr}")]
    MyEndpoint { addr: UnknownAddr },

    #[error("Can't send a message through udp socket, err: {err}")]
    MsgSendFail { err: String },

    #[error("Peer socket addr create fail, err: {err}")]
    MalformedAddr { err: String },

    #[error(
        "Addr has already been discovered and is mapped, disc_endpoint: \
        {disc_endpoint}"
    )]
    AddrAlreadyMapped { disc_endpoint: String },
}

pub(crate) async fn init_who_are_you(
    unknown_addr: UnknownAddr,
    identity: Arc<Identity>,
    table: Arc<Table>,
    udp_conn: Arc<Connection>,
) -> Result<(), WhoAreYouInitError> {
    let her_disc_endpoint = unknown_addr.disc_endpoint();
    let my_disc_port = identity.disc_port;

    if check::is_my_endpoint(my_disc_port, &unknown_addr.disc_endpoint()) {
        return Err(WhoAreYouInitError::MyEndpoint { addr: unknown_addr });
    }

    let table = table.clone();

    if let Some(_) = table.get_mapped_addr_lock(&her_disc_endpoint).await {
        return Err(WhoAreYouInitError::AddrAlreadyMapped {
            disc_endpoint: her_disc_endpoint.to_string(),
        });
    };

    let src_disc_port = identity.disc_port;
    let src_p2p_port = identity.p2p_port;
    let src_sig = identity.credential.sig;
    let src_public_key_str = identity.credential.public_key_str.clone();

    let way = WhoAreYou {
        src_sig,
        src_disc_port,
        src_p2p_port,
        src_public_key_str,
    };

    let mut tx_lock = udp_conn.tx.write().await;

    let her_socket_addr: SocketAddr = match &her_disc_endpoint.parse() {
        Ok(a) => *a,
        Err(err) => {
            return Err(WhoAreYouInitError::MalformedAddr {
                err: err.to_string(),
            });
        }
    };

    match tx_lock
        .send((Msg::WhoAreYouSyn(way), her_socket_addr))
        .await
    {
        Ok(_) => {
            tdebug!(
                "p2p_discovery",
                "whoareyou",
                "WhoAreYou SYN has been successfully sent, to: {}",
                &her_disc_endpoint,
            );
        }
        Err(err) => {
            return Err(WhoAreYouInitError::MsgSendFail {
                err: err.to_string(),
            });
        }
    };

    Ok(())
}
