use super::{check, WHO_ARE_YOU_EXPIRATION_SEC};
use crate::{
    msg::WhoAreYou,
    state::DiscState,
    table::{Addr, AddrSlot, Slot, SlotGuard, Table, UnknownAddrNode},
};
use logger::tdebug;
use p2p_identity::addr::{KnownAddrStatus, UnknownAddr};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{mpsc, RwLock};

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
        "Previous WhoAreYou succeeded entry still lives in the table, \
        disc_endpoint: {disc_endpoint}"
    )]
    WhoAreYouNotExpired { disc_endpoint: String },
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
            match &*addr_lock {
                Addr::Unknown(u) => {}
                Addr::Known(n) => {
                    if let KnownAddrStatus::WhoAreYouSuccess { at } =
                        n.known_addr.status
                    {
                        if !check::is_who_are_you_expired(
                            WHO_ARE_YOU_EXPIRATION_SEC,
                            at,
                        ) {
                            return Err(
                                WhoAreYouInitError::WhoAreYouNotExpired {
                                    disc_endpoint: n.known_addr.disc_endpoint(),
                                },
                            );
                        }
                    }
                }
            }

            AddrSlot::Addr(addr_lock, addr)
        }
        None => match table.get_empty_slot().await {
            Ok(s) => AddrSlot::Slot(s),
            Err(err) => {
                return Err(WhoAreYouInitError::AddrSlotReserveFail { err });
            }
        },
    };

    // match &*node_lock {
    //     AddrNode::Known(known_addr) => {
    //         if !check::is_who_are_you_expired(
    //             WHO_ARE_YOU_EXPIRATION_SEC,
    //             known_addr.known_at,
    //         ) {
    //             return Err(WhoAreYouInitError::WhoAreYouNotExpired {
    //                 disc_endpoint: known_addr.disc_endpoint(),
    //             });
    //         }
    //     }
    //     _ => {}
    // };

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

    let way_syn_msg = match way.into_syn_msg() {
        Ok(m) => m,
        Err(err) => return Err(WhoAreYouInitError::MsgCreateFail { err }),
    };

    match disc_state
        .udp_conn
        .write_msg(&her_disc_endpoint, way_syn_msg)
        .await
    {
        Ok(_) => {
            match addr_slot {
                AddrSlot::Slot(s) => {
                    let unknown_addr_node = {
                        let a = Addr::Unknown(UnknownAddrNode {
                            unknown_addr,
                            __internal_slot: s,
                        });
                        Arc::new(RwLock::new(a))
                    };

                    table
                        .insert_mapping(&her_disc_endpoint, unknown_addr_node)
                        .await;
                }
                AddrSlot::Addr(mut addr_lock, addr) => {
                    // let unknown_addr_node = {
                    //     let a = Addr::Unknown(UnknownAddrNode {
                    //         unknown_addr,
                    //         __internal_slot: s,
                    //     });
                    //     Arc::new(RwLock::new(a))
                    // };

                    // pub ip: String,
                    // pub disc_port: u16,
                    // pub p2p_port: Option<u16>,
                    // pub public_key_str: Option<String>,
                    // Some(1).take()
                    match &mut *addr_lock {
                        Addr::Known(k) => {
                            let known_addr = &k.known_addr;
                            let (tx, rx) = mpsc::unbounded_channel();
                            let slot_guard = SlotGuard {
                                slot: Arc::new(Slot { idx: 0 }),
                                slots_tx: Arc::new(tx),
                            };

                            let a = std::mem::replace(
                                &mut k.__internal_slot,
                                slot_guard,
                            );

                            std::mem::forget(&slot_guard);
                            *addr_lock = Addr::Unknown(UnknownAddrNode {
                                unknown_addr,
                                __internal_slot: a,
                            });
                        }
                        Addr::Unknown(u) => (),
                    };
                }
            };

            // *node_lock = AddrNode::Unknown(addr);

            // table.insert_mapping(&her_disc_endpoint, node).await;

            tdebug!(
                "p2p_discovery",
                "whoareyou",
                "Addr updated after whoareyou initiate"
            );
        }
        Err(err) => return Err(WhoAreYouInitError::MsgSendFail { err }),
    };

    Ok(())
}
