use super::{
    check::{self, WHO_ARE_YOU_EXPIRATION_SEC},
    WhoAreYou,
};
use crate::{
    v1::{net::Connection, ops::Msg},
    Addr, AddrSlot, Table,
};
use chrono::Utc;
use colored::Colorize;
use futures::sink::SinkExt;
use logger::{tdebug, terr};
use p2p_addr::{AddrStatus, KnownAddr};
use p2p_identity::Identity;
use std::{net::SocketAddr, sync::Arc};
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub(crate) enum WhoAreYouRecvError {
    #[error("Can't take request I sent, addr: {addr}")]
    MyEndpoint { addr: KnownAddr },

    #[error("Can't send a message through udp socket, err: {err}")]
    MsgSendFail { err: String },

    #[error("Could not register as a known node, endpoint: {disc_endpoint}")]
    KnownNodeRegisterFail { disc_endpoint: String, err: String },

    #[error("Could not make public key out of string: {public_key_str}")]
    PublicKeyCreateFail { public_key_str: String, err: String },

    #[error(
        "Addr has already been discovered and is mapped, endpoint: \
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
    table: Arc<Table>,
) -> Result<(), WhoAreYouRecvError> {
    let WhoAreYou {
        src_sig: her_sig,
        src_disc_port: her_disc_port,
        src_p2p_port: her_p2p_port,
        src_public_key_str: her_public_key_str,
    } = way_syn;

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
        public_key_str: her_public_key_str,
        public_key: her_public_key,
        status: AddrStatus::WhoAreYouSynRecv { at: Utc::now() },
    };

    let her_disc_endpoint = known_addr.disc_endpoint();
    let her_p2p_endpoint = known_addr.p2p_endpoint();

    if check::is_my_endpoint(identity.disc_port, &her_disc_endpoint) {
        return Err(WhoAreYouRecvError::MyEndpoint { addr: known_addr });
    }

    let slot = match table.get_mapped_addr_lock(&her_disc_endpoint).await {
        Some(_) => {
            return Err(WhoAreYouRecvError::AddrAlreadyMapped {
                disc_endpoint: her_disc_endpoint.to_string(),
            });
        }
        None => match table.get_empty_slot().await {
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

    let her_disc_endpoint: SocketAddr = match her_disc_endpoint.parse() {
        Ok(a) => a,
        Err(err) => {
            return Err(WhoAreYouRecvError::EndpointParseFail {
                err: err.to_string(),
            });
        }
    };

    match tx_lock
        .send((Msg::WhoAreYouAck(way_ack), her_disc_endpoint))
        .await
    {
        Ok(_) => {
            let addr = {
                let a = Addr {
                    known_addr,
                    addr_slot_guard: slot,
                };

                Arc::new(RwLock::new(a))
            };

            table
                .insert_mapping(&her_disc_endpoint.to_string(), addr.clone())
                .await;

            match table.enqueue_known_addr(addr).await {
                Ok(_) => {
                    tdebug!(
                        "p2p_discovery",
                        "whoareyou",
                        "Enqueueing known addr, p2p endpoint: {}",
                        her_p2p_endpoint.green(),
                    );
                }
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
            };
        }
        Err(err) => {
            return Err(WhoAreYouRecvError::MsgSendFail {
                err: err.to_string(),
            });
        }
    };

    Ok(())
}
