use super::{AppState, KeyedAction};
use crate::EnvelopeError;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, error::SendError, Receiver, Sender},
    RwLock, RwLockWriteGuard,
};

pub(crate) struct Dispatcher {
    state: Arc<RwLock<AppState>>,
    action_bus_rx: Receiver<Action>,
    action_bus_tx: Sender<Action>,
}

impl Dispatcher {
    pub fn new(
        state: Arc<RwLock<AppState>>,
    ) -> Result<Dispatcher, EnvelopeError> {
        let (action_bus_tx, action_bus_rx) = {
            let (tx, rx) = mpsc::channel::<Action>(100);
            (tx, rx)
        };

        let d = Dispatcher {
            state,
            action_bus_rx,
            action_bus_tx,
        };

        Ok(d)
    }

    pub async fn run(&mut self) {
        while let Some(action) = self.action_bus_rx.recv().await {
            // handler.handle_io_event(io_event).await;
            let state = self.state.write().await;
            self.reduce(state, action);
        }
    }

    pub async fn dispatch(
        &self,
        action: Action,
    ) -> Result<(), SendError<Action>> {
        self.action_bus_tx.send(action).await
    }

    pub fn reduce<'a>(
        &self,
        mut state: RwLockWriteGuard<'a, AppState>,
        action: Action,
    ) {
    }
}
