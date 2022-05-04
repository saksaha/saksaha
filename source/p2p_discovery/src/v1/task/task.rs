use crate::state::DiscState;
use p2p_identity::addr::Addr;
use std::sync::Arc;

pub(crate) type DiscoveryTaskInstance = TaskInstance<DiscoveryTask>;

#[derive(Clone)]
pub(crate) struct TaskInstance<T> {
    pub(crate) task: Arc<T>,
    pub(crate) fail_count: usize,
}

impl<T> TaskInstance<T> {
    pub fn new(task: T) -> Self {
        TaskInstance {
            task: Arc::new(task),
            fail_count: 0,
        }
    }
}

pub(crate) enum TaskResult {
    Success,
    Fail,
    FailRetry { msg: String },
}

#[derive(Clone)]
pub(crate) enum DiscoveryTask {
    InitiateWhoAreYou {
        addr: Addr,
        disc_state: Arc<DiscState>,
    },
}

impl<T> std::fmt::Display for TaskInstance<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, fail_count: {}", self.task, self.fail_count)
    }
}

impl std::fmt::Display for DiscoveryTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitiateWhoAreYou { addr, .. } => {
                write!(f, "InitiateWhoAreYou, addr: {:?}", addr)
            }
        }
    }
}
