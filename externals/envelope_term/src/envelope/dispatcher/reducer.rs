use crate::{
    envelope::{Action, AppState},
    EnvelopeError,
};
use log::info;
use std::time::Duration;
use tokio::sync::RwLockWriteGuard;

pub(crate) struct Reducer;

impl Reducer {
    pub async fn reduce<'a>(
        &self,
        mut state: RwLockWriteGuard<'a, AppState>,
        action: Action,
    ) -> Result<(), EnvelopeError> {
        log::info!("reduce!!, action: {}", action);

        match action {
            Action::Initialize => do_initialize(state).await?,
            Action::GetChList(data) => get_ch_list(state, data).await?,
            Action::GetMessages(data) => get_msgs(state, data).await?,
            _ => info!("Currently not handled!!"),
        };

        Ok(())
    }
}

/// We use dummy implementation here, just wait 1s
async fn do_initialize<'a>(
    mut state: RwLockWriteGuard<'a, AppState>,
) -> Result<(), EnvelopeError> {
    info!("🚀 Initializing the application, waiting for 1 second");

    tokio::time::sleep(Duration::from_secs(1)).await;

    // let mut state = self.envelope.get_state().write().await;

    state.set_is_initialized(true);

    info!("👍 Application initialized");

    Ok(())
}

async fn get_ch_list<'a>(
    mut state: RwLockWriteGuard<'a, AppState>,
    data: Vec<u8>,
) -> Result<(), EnvelopeError> {
    // self.envelope.set_ch_list(data).await?;

    state.set_ch_list(data);

    Ok(())
}

async fn get_msgs<'a>(
    mut state: RwLockWriteGuard<'a, AppState>,
    data: Vec<u8>,
) -> Result<(), EnvelopeError> {
    // let mut app = self.app.lock().await;

    // self.envelope.set_chats(data).await?;

    Ok(())
}
