use crate::envelope::dispatcher::{Dispatch, Dispatcher};
use crate::envelope::{actions, AppReturn, AppState, View};
use crate::io::InputMode;
use crate::Envelope;
use crate::{envelope::Action, inputs::key::Key};
use log::{info, warn};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::RwLockWriteGuard;

impl Envelope {
    pub async fn handle_key_input_in_ch_list<'a>(
        &self,
        key: Key,
        state: RwLockWriteGuard<'a, AppState>,
    ) -> AppReturn {
        match key {
            Key::Char('1') => (),
            Key::Char('2') => (),
            Key::Char('3') => (),
        };

        AppReturn::Continue
    }
}
