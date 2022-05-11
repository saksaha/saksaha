use super::DiscoveryTask;
use crate::{ops::whoareyou::WhoAreYouInitError, v1::ops::whoareyou};
use logger::twarn;

pub(crate) async fn run(task: DiscoveryTask) {
    match task {
        DiscoveryTask::InitiateWhoAreYou { addr, disc_state } => {
            match whoareyou::init_who_are_you(addr.clone(), disc_state.clone())
                .await
            {
                Ok(_) => {}
                Err(err) => {
                    match err {
                        WhoAreYouInitError::MyEndpoint { .. } => {
                            twarn!(
                                "p2p_discovery",
                                "task",
                                "Abandoning failed task, err: {}",
                                err
                            );
                        }
                        _ => {
                            twarn!(
                                "p2p_discovery",
                                "task",
                                "Unhandled error, err: {}",
                                err,
                            );
                        }
                    };
                }
            }
        }
    };
}
