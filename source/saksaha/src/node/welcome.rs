use std::time::Duration;

use crate::{machine::Machine, system::BoxedError};
use futures::{SinkExt, StreamExt};
use log::{info, warn};
use sak_p2p_trpt::{BlockHeightSynMsg, Msg, UpgradedConnection};
use tokio::sync::RwLockWriteGuard;

use super::event_handle::handle_new_blocks_ev;

const RESPONSE_TIMEOUT: u64 = 2000;

pub(crate) async fn handle_welcome<'a>(
    public_key: &str,
    conn: &mut RwLockWriteGuard<'_, UpgradedConnection>,
    machine: &Machine,
) -> Result<(), BoxedError> {
    let latest_height = machine
        .blockchain
        .dist_ledger
        .get_latest_block_height()
        .await?
        .unwrap_or(0);

    conn.socket
        .send(Msg::BlockHeightSyn(BlockHeightSynMsg {
            block_height: latest_height,
        }))
        .await?;

    println!("syn send ok");

    let resp_timeout =
        tokio::time::sleep(Duration::from_millis(RESPONSE_TIMEOUT));

    let block_hashes = tokio::select! {
        _ = resp_timeout => {
            return Err(format!("Peer did not respond in time, dst public_key: {}",
                public_key,).into());
        },
        resp = conn.socket.next() => {
            match resp {
                Some(maybe_msg) => match maybe_msg {
                    Ok(msg) => match msg {
                        Msg::BlockHeightAck(h) => {
                            let mut block_hashes = vec![];
                            if latest_height > h.block_height {

                                for i in h.block_height..latest_height {
                                    let block_hash = match machine
                                    .blockchain
                                    .dist_ledger
                                    .get_block_hash_by_height(&i)
                                    .await? {
                                        Some(h) => h,
                                        None => {
                                            warn!("There is no matched \
                                            tx in my ledger");
                                            continue
                                        },
                                    };

                                    block_hashes.push((format!("{}", i), block_hash));
                                }
                            }

                            block_hashes
                        }
                        other_msg => {
                            return Err(format!(
                                "Received an invalid type message, msg: {:?}",
                                other_msg).into());

                        }
                    },
                    Err(err) => {
                        return Err(format!(
                                "Failed to parse the msg, err: {}", err).into());
                    }
                },
                None => {
                    return Err(format!(
                                "Received an invalid data stream").into());

                }
            }
        },
    };

    handle_new_blocks_ev(public_key, conn, machine, block_hashes).await;

    Ok(())
}
