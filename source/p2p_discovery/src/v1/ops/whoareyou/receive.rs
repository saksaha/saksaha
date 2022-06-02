use super::{check, WhoAreYou};
use crate::{
    v1::{net::Connection, ops::Msg},
    Addr, AddrTable,
};
use chrono::Utc;
use futures::sink::SinkExt;
use logger::terr;
use p2p_addr::{AddrStatus, KnownAddr};
use p2p_identity::Identity;
use std::{net::SocketAddr, sync::Arc};
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub(crate) enum WhoAreYouRecvError {
    #[error("Will not proceed with disc_endpoint being myself")]
    MyEndpoint,

    #[error("Can't send a message through udp socket, err: {err}")]
    MsgSendFail { err: String },

    #[error("Could not register as a known node, endpoint: {disc_endpoint}")]
    KnownNodeRegisterFail { disc_endpoint: String, err: String },

    #[error("Could not make public key out of string: {public_key_str}")]
    PublicKeyCreateFail { public_key_str: String, err: String },

    #[error(
        "Addr has already been discovered and is mapped, disc_endpoint: \
        {disc_endpoint}"
    )]
    AddrAlreadyMapped { disc_endpoint: String },

    #[error("Could not reserve addr slot")]
    AddrSlotReserveFail,

    #[error("Could not parse her endpoint into SocketAddr, err: {err}")]
    EndpointParseFail { err: String },
}

pub(crate) async fn recv_who_are_you(
    socket_addr: SocketAddr,
    udp_conn: Arc<Connection>,
    way_syn: WhoAreYou,
    identity: Arc<Identity>,
    addr_table: Arc<AddrTable>,
) -> Result<(), WhoAreYouRecvError> {
    let WhoAreYou {
        src_sig: her_sig,
        src_disc_port: her_disc_port,
        src_p2p_port: her_p2p_port,
        src_public_key_str: her_public_key_str,
    } = way_syn;

    let her_disc_endpoint =
        utils_net::make_endpoint(&socket_addr.ip().to_string(), her_disc_port);

    if check::is_my_endpoint(identity.disc_port, &her_disc_endpoint) {
        return Err(WhoAreYouRecvError::MyEndpoint);
    }

    let slot_guard = match addr_table.get_mapped_addr(&her_public_key_str).await
    {
        Some(_) => {
            return Err(WhoAreYouRecvError::AddrAlreadyMapped {
                disc_endpoint: her_disc_endpoint.to_string(),
            });
        }
        None => match addr_table.get_empty_slot().await {
            Ok(s) => s,
            Err(_) => {
                return Err(WhoAreYouRecvError::AddrSlotReserveFail);
            }
        },
    };

    let my_disc_port = identity.disc_port;
    let my_p2p_port = identity.p2p_port;
    let my_sig = identity.credential.sig;
    let my_public_key_str = identity.credential.public_key_str.clone();

    let way_ack = WhoAreYou {
        src_sig: my_sig,
        src_disc_port: my_disc_port,
        src_p2p_port: my_p2p_port,
        src_public_key_str: my_public_key_str,
    };

    let mut tx_lock = udp_conn.tx.write().await;

    let her_disc_socket_addr: SocketAddr = match her_disc_endpoint.parse() {
        Ok(a) => a,
        Err(err) => {
            return Err(WhoAreYouRecvError::EndpointParseFail {
                err: err.to_string(),
            });
        }
    };

    if let Err(err) = tx_lock
        .send((Msg::WhoAreYouAck(way_ack), her_disc_socket_addr))
        .await
    {
        return Err(WhoAreYouRecvError::MsgSendFail {
            err: err.to_string(),
        });
    }

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

    let known_addr = KnownAddr {
        ip: socket_addr.ip().to_string(),
        disc_port: her_disc_port,
        p2p_port: her_p2p_port,
        sig: her_sig,
        public_key_str: her_public_key_str.clone(),
        public_key: her_public_key,
        status: RwLock::new(AddrStatus::WhoAreYouSuccess { at: Utc::now() }),
    };

    let addr = {
        let a = Addr {
            known_addr,
            _addr_slot_guard: slot_guard,
        };

        Arc::new(a)
    };

    match addr_table.insert_mapping(addr.clone()).await {
        Ok(_) => {}
        Err(err) => {
            terr!(
                "p2p_discovery",
                "whoareyou",
                "Fail to add known node. Queue might have been closed",
            );

            return Err(WhoAreYouRecvError::KnownNodeRegisterFail {
                disc_endpoint: her_disc_endpoint.to_string(),
                err,
            });
        }
    }

    Ok(())
}
