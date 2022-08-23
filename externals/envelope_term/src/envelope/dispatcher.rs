use crate::EnvelopeError;

use super::{Action, AppState};
use std::sync::Arc;
use tokio::sync::{RwLock, RwLockWriteGuard};

pub(crate) struct Dispatcher {
    state: Arc<RwLock<AppState>>,
}

impl Dispatcher {
    pub fn new(
        state: Arc<RwLock<AppState>>,
    ) -> Result<Dispatcher, EnvelopeError> {
        let d = Dispatcher { state };

        Ok(d)
    }

    pub async fn dispatch(&self, action: Action) {
        let state = self.state.write().await;

        self.reduce(state, action);
    }

    pub async fn reduce<'a>(
        &self,
        state: RwLockWriteGuard<'a, AppState>,
        action: Action,
    ) {
    }
}
