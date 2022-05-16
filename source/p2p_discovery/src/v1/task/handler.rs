use super::DiscoveryTask;
use crate::v1::ops::whoareyou;
use logger::tdebug;

pub(crate) async fn run(task: DiscoveryTask) {
    match task {
        DiscoveryTask::InitiateWhoAreYou { addr, disc_state } => {
            match whoareyou::init_who_are_you(addr.clone(), disc_state.clone())
                .await
            {
                Ok(_) => {}
                Err(err) => {
                    match err {
                        _ => {
                            tdebug!(
                                "p2p_discovery",
                                "task",
                                "WhoAreYouInit stopped, err: {}",
                                err,
                            );
                        }
                    };
                }
            }
        }
    };
}
