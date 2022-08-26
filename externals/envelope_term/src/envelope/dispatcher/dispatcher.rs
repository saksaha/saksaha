use crate::{
    envelope::{
        reducer::{DispatcherContext, Reducer},
        Action, AppState,
    },
    EnvelopeError,
};
use std::{future::Future, pin::Pin, sync::Arc};
use tokio::sync::{
    mpsc::{self, error::SendError, Receiver, Sender},
    Mutex, RwLock,
};

pub(crate) type Dispatch = Box<
    dyn Fn(
            Action,
        ) -> Pin<
            Box<
                dyn Future<Output = Result<(), SendError<Action>>>
                    + Send
                    + Sync,
            >,
        > + Send
        + Sync,
>;

pub(crate) struct Dispatcher {
    state: Arc<RwLock<AppState>>,
    action_bus_rx: Mutex<Receiver<Action>>,
    action_bus_tx: Sender<Action>,
    reducer: Reducer,
    ctx: Arc<DispatcherContext>,
}

impl Dispatcher {
    pub fn new(
        state: Arc<RwLock<AppState>>,
        ctx: Arc<DispatcherContext>,
    ) -> Result<Dispatcher, EnvelopeError> {
        let (action_bus_tx, action_bus_rx) = {
            let (tx, rx) = mpsc::channel::<Action>(100);
            (tx, Mutex::new(rx))
        };

        let reducer = Reducer {};

        let d = Dispatcher {
            state,
            action_bus_rx,
            action_bus_tx,
            reducer,
            ctx,
        };

        Ok(d)
    }

    pub async fn run(&self) {
        let mut action_bus_rx_lock = self.action_bus_rx.lock().await;

        while let Some(action) = action_bus_rx_lock.recv().await {
            // handler.handle_io_event(io_event).await;
            let state = self.state.write().await;
            self.reducer.reduce(state, action, &self.ctx);
        }
    }

    pub async fn dispatch(
        &self,
        action: Action,
    ) -> Result<(), SendError<Action>> {
        self.action_bus_tx.send(action).await
    }

    pub fn get_context(&self) -> &Arc<DispatcherContext> {
        &self.ctx
    }
}
