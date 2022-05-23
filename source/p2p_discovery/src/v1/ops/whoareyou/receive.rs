use super::{check, WHO_ARE_YOU_EXPIRATION_SEC};
use crate::{
    msg::{Msg2, WhoAreYou},
    state::DiscState,
    table::{Addr, AddrSlot, AddrVal},
};
use chrono::Utc;
use colored::Colorize;
use futures::{SinkExt, StreamExt};
use logger::{tdebug, terr};
use p2p_identity::addr::{AddrStatus, KnownAddr};
use std::{net::SocketAddr, str::FromStr, sync::Arc};
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub(crate) enum WhoAreYouRecvError {
    #[error("Can't take request I sent, addr: {addr}")]
    MyEndpoint { addr: KnownAddr },

    #[error("Can't make a message (WhoAreYouAck), err: {err}")]
    MsgCreateFail { err: String },

    #[error("Can't send a message through udp socket, err: {err}")]
    MsgSendFail { err: String },

    #[error("Could not register as a known node, endpoint: {disc_endpoint}")]
    KnownNodeRegisterFail { disc_endpoint: String, err: String },

    #[error("Could not make public key out of string: {public_key_str}")]
    PublicKeyCreateFail { public_key_str: String, err: String },

    #[error(
        "Previous WhoAreYou succeeded entry still lives in the table, \
        disc_endpoint: {disc_endpoint}"
    )]
    WhoAreYouNotExpired { disc_endpoint: String },

    #[error("Could not reserve addr node")]
    AddrNodeReserveFail,

    #[error("Could not parse her endpoint into SocketAddr, err: {err}")]
    EndpointParseFail { err: String },
}

pub(crate) async fn recv_who_are_you(
    socket_addr: SocketAddr,
    disc_state: Arc<DiscState>,
    way_syn: WhoAreYou,
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

    if check::is_my_endpoint(disc_state.disc_port, &her_disc_endpoint) {
        return Err(WhoAreYouRecvError::MyEndpoint { addr: known_addr });
    }

    let addr_slot = match disc_state
        .table
        .get_mapped_addr_lock(&her_disc_endpoint)
        .await
    {
        Some((addr_lock, addr)) => AddrSlot::Addr(addr_lock, addr),
        None => match disc_state.table.get_empty_slot().await {
            Ok(s) => AddrSlot::Slot(s),
            Err(_) => {
                return Err(WhoAreYouRecvError::AddrNodeReserveFail);
            }
        },
    };

    if let AddrSlot::Addr(addr_lock, addr) = &addr_slot {
        if let AddrVal::Known(known_addr) = &addr_lock.val {
            if let AddrStatus::WhoAreYouSuccess { at } = known_addr.status {
                if !check::is_who_are_you_expired(
                    WHO_ARE_YOU_EXPIRATION_SEC,
                    at,
                ) {
                    return Err(WhoAreYouRecvError::WhoAreYouNotExpired {
                        disc_endpoint: known_addr.disc_endpoint(),
                    });
                }
            }
        }
    }

    let my_disc_port = disc_state.disc_port;
    let my_p2p_port = disc_state.p2p_port;
    let my_sig = disc_state.p2p_identity.sig;
    let my_public_key_str = disc_state.p2p_identity.public_key_str.clone();

    let way = WhoAreYou {
        src_sig: my_sig,
        src_disc_port: my_disc_port,
        src_p2p_port: my_p2p_port,
        src_public_key_str: my_public_key_str,
    };

    let way_msg = match way.into_ack_msg() {
        Ok(m) => m,
        Err(err) => {
            return Err(WhoAreYouRecvError::MsgCreateFail { err });
        }
    };

    let msg2 = Msg2::WhoAreYou(way);

    let mut tx_lock = disc_state.udp_conn.tx.write().await;

    // let socket_addr =
    //     SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 35518);

    let her_disc_endpoint: SocketAddr = match her_disc_endpoint.parse() {
        Ok(a) => a,
        Err(err) => {
            return Err(WhoAreYouRecvError::EndpointParseFail {
                err: err.to_string(),
            });
        }
    };

    match tx_lock.send((msg2, her_disc_endpoint)).await {
        Ok(_) => {
            let addr = match addr_slot {
                AddrSlot::Slot(s) => {
                    let addr = {
                        let a = Addr {
                            val: AddrVal::Known(known_addr),
                            __internal_slot: s,
                        };

                        Arc::new(RwLock::new(a))
                    };

                    disc_state
                        .table
                        .insert_mapping(
                            &her_disc_endpoint.to_string(),
                            addr.clone(),
                        )
                        .await;

                    addr
                }

                // Addr that we've known a long time ago
                AddrSlot::Addr(mut addr_lock, addr) => {
                    addr_lock.val = AddrVal::Known(known_addr);
                    addr.clone()
                }
            };

            match disc_state.table.enqueue_known_addr(addr).await {
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
