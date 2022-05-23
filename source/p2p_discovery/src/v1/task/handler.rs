use super::DiscoveryTask;
use crate::v1::ops::whoareyou;
use logger::tdebug;

pub(crate) async fn run(task: DiscoveryTask) {
    match task {
        DiscoveryTask::InitiateWhoAreYou { addr, disc_state } => {
            let disc_endpoint = addr.disc_endpoint();

            match whoareyou::init_who_are_you(addr, disc_state.clone()).await {
                Ok(_) => {}
                Err(err) => {
                    match err {
                        _ => {
                            tdebug!(
                                "p2p_discovery",
                                "task",
                                "WhoAreYouInit stopped, err: {}, \
                                disc_endpoint: {}",
                                err,
                                disc_endpoint,
                            );
                        }
                    };
                }
            }
        }
    };
}
