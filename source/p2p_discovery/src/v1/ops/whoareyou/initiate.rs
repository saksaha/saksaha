use crate::{
    v1::{ops::Msg, state::DiscState},
    Addr, AddrSlot, AddrVal,
};

use super::{
    check::{self, WHO_ARE_YOU_EXPIRATION_SEC},
    WhoAreYou,
};
use chrono::{DateTime, Utc};
use futures::{SinkExt, StreamExt};
use logger::tdebug;
use p2p_identity::addr::{AddrStatus, KnownAddr, UnknownAddr};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub(crate) enum WhoAreYouInitError {
    #[error("Can't send request to myself, addr: {addr}")]
    MyEndpoint { addr: UnknownAddr },

    #[error("Can't make a message (WhoAreYouAck), err: {err}")]
    MsgCreateFail { err: String },

    #[error("Can't send a message through udp socket, err: {err}")]
    MsgSendFail { err: String },

    #[error("No available addr slot, err: {err}")]
    AddrSlotReserveFail { err: String },

    #[error(
        "Addr is already valid and fresh but attempt to modify has \
        been made"
    )]
    AttemptToUpdateValidAddr,

    #[error(
        "Successfuly WhoAreYou has been made recently, \
        at: {at}"
    )]
    WhoAreYouNotExpired { at: DateTime<Utc> },
}

pub(crate) async fn init_who_are_you(
    unknown_addr: UnknownAddr,
    disc_state: Arc<DiscState>,
) -> Result<(), WhoAreYouInitError> {
    let her_disc_endpoint = unknown_addr.disc_endpoint();
    let my_disc_port = disc_state.disc_port;

    if check::is_my_endpoint(my_disc_port, &unknown_addr.disc_endpoint()) {
        return Err(WhoAreYouInitError::MyEndpoint { addr: unknown_addr });
    }

    let table = disc_state.table.clone();

    let addr_slot = match table.get_mapped_addr_lock(&her_disc_endpoint).await {
        Some((addr_lock, addr)) => {
            match &addr_lock.val {
                AddrVal::Unknown(_) => AddrSlot::Addr(addr_lock, addr),
                AddrVal::Known(known_addr) => {
                    if let AddrStatus::WhoAreYouSuccess { at } =
                        known_addr.status
                    {
                        if !check::is_who_are_you_expired(
                            WHO_ARE_YOU_EXPIRATION_SEC,
                            at,
                        ) {
                            return Err(
                                WhoAreYouInitError::WhoAreYouNotExpired { at },
                            );
                        } else {
                            // WhoAreYou succeeded long time ago (old)
                            table.remove_mapping(&her_disc_endpoint).await;

                            match table.get_empty_slot().await {
                                Ok(s) => AddrSlot::Slot(s),
                                Err(err) => {
                                    return Err(
                                WhoAreYouInitError::AddrSlotReserveFail {
                                    err,
                                },
                            );
                                }
                            }
                        }
                    } else {
                        // Previous WhoAreYou not successful
                        AddrSlot::Addr(addr_lock, addr)
                    }
                }
            }
        }
        None => match table.get_empty_slot().await {
            Ok(s) => AddrSlot::Slot(s),
            Err(err) => {
                return Err(WhoAreYouInitError::AddrSlotReserveFail { err });
            }
        },
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

    // let way_syn_frame = match way.into_frame() {
    //     Ok(m) => m,
    //     Err(err) => return Err(WhoAreYouInitError::MsgCreateFail { err }),
    // };

    let mut tx_lock = disc_state.udp_conn.tx.write().await;

    let socket_addr =
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 35518);

    match tx_lock.send((Msg::WhoAreYouSyn(way), socket_addr)).await {
        Ok(_) => {
            match addr_slot {
                // Fresh new attempt OR expired destination
                AddrSlot::Slot(s) => {
                    let unknown_addr_node = {
                        let addr = Addr {
                            val: AddrVal::Unknown(unknown_addr),
                            __internal_slot: s,
                        };
                        Arc::new(RwLock::new(addr))
                    };

                    table
                        .insert_mapping(&her_disc_endpoint, unknown_addr_node)
                        .await;
                }

                // Previous unsuccessful WhoAreYou attempt
                AddrSlot::Addr(mut addr_lock, addr) => {
                    match &mut addr_lock.val {
                        AddrVal::Unknown(ua) => {
                            *ua = unknown_addr;
                            ua.status =
                                AddrStatus::WhoAreYouInit { at: Utc::now() };
                        }
                        _ => {
                            return Err(
                                WhoAreYouInitError::AttemptToUpdateValidAddr,
                            );
                        }
                    };
                }
            };

            tdebug!(
                "p2p_discovery",
                "whoareyou",
                "Addr updated after whoareyou initiate"
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
