use super::{check, WhoAreYou, WhoAreYouRecvError};
use crate::{
    v1::{net::Connection, ops::Msg},
    Addr, AddrTable,
};
use colored::Colorize;
use futures::SinkExt;
use logger::{tdebug, terr};
use p2p_addr::{AddrStatus, KnownAddr, UnknownAddr};
use p2p_identity::Identity;
use std::{net::SocketAddr, sync::Arc};
use thiserror::Error;
use tokio::sync::RwLock;

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
    addr_table: Arc<AddrTable>,
    udp_conn: Arc<Connection>,
) -> Result<(), WhoAreYouInitError> {
    let her_disc_endpoint = unknown_addr.disc_endpoint();
    let my_disc_port = identity.disc_port;

    if check::is_my_endpoint(my_disc_port, &unknown_addr.disc_endpoint()) {
        return Err(WhoAreYouInitError::MyEndpoint { addr: unknown_addr });
    }

    if let Some(ref public_key_str) = unknown_addr.public_key_str {
        if let Some(_) = addr_table.get_mapped_addr(public_key_str).await {
            return Err(WhoAreYouInitError::AddrAlreadyMapped {
                disc_endpoint: her_disc_endpoint.to_string(),
            });
        }
    }

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
            // tdebug!(
            //     "p2p_discovery",
            //     "whoareyou",
            //     "WhoAreYou SYN has been successfully sent, to: {}",
            //     &her_disc_endpoint,
            // );
        }
        Err(err) => {
            return Err(WhoAreYouInitError::MsgSendFail {
                err: err.to_string(),
            });
        }
    };

    Ok(())
}

pub(crate) async fn handle_who_are_you_ack(
    way_ack: WhoAreYou,
    socket_addr: SocketAddr,
    udp_conn: Arc<Connection>,
    identity: Arc<Identity>,
    addr_table: Arc<AddrTable>,
) -> Result<(), String> {
    let WhoAreYou {
        src_sig: her_sig,
        src_disc_port: her_disc_port,
        src_p2p_port: her_p2p_port,
        src_public_key_str: her_public_key_str,
    } = way_ack;

    if let Some(_) = addr_table.get_mapped_addr_lock(&her_public_key_str).await
    {
        return Err(format!("Address is already mapped."));
    };

    let her_public_key = match crypto::convert_public_key_str_into_public_key(
        &her_public_key_str,
    ) {
        Ok(p) => p,
        Err(err) => return Err(err),
    };

    let slot_guard = addr_table.get_empty_slot().await?;

    let known_addr = KnownAddr {
        ip: socket_addr.ip().to_string(),
        disc_port: her_disc_port,
        p2p_port: her_p2p_port,
        sig: her_sig,
        public_key_str: her_public_key_str.clone(),
        public_key: her_public_key,
        status: AddrStatus::WhoAreYouInProgress,
    };

    let her_disc_endpoint = known_addr.disc_endpoint();
    let her_p2p_endpoint = known_addr.p2p_endpoint();
    let her_public_key_str = known_addr.public_key_str.clone();

    let addr = {
        let a = Addr {
            known_addr,
            addr_slot_guard: slot_guard,
        };

        Arc::new(RwLock::new(a))
    };

    match addr_table.insert_mapping(addr).await {
        Ok(_) => {
            // tdebug!(
            //     "p2p_discovery",
            //     "server",
            //     "Whoareyou Success! p2p_endpoint: {}, \
            //                     disc_endpoint: {}",
            //     her_p2p_endpoint.green(),
            //     her_disc_endpoint.green(),
            // );
        }
        Err(_) => {
            terr!(
                "p2p_discovery",
                "server",
                "Fail to add known node. Queue might have been \
                                closed",
            );
        }
    };

    Ok(())
}
