use super::actions::Actions;
use super::state::AppState;
use crate::app::actions::Action;
use crate::inputs::key::Key;
use crate::io::InputMode;
use crate::io::IoEvent;

use log::{debug, error, warn};

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App {
    io_tx: tokio::sync::mpsc::Sender<IoEvent>,
    actions: Actions,
    state: AppState,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IoEvent>) -> Self {
        let actions = vec![Action::Quit].into();
        let state = AppState::default();

        Self {
            io_tx,
            actions,
            state,
        }
    }

    pub async fn handle_normal_key(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            debug!("Run action [{:?}]", action);
            match action {
                Action::Quit => AppReturn::Exit,
                Action::Sleep => AppReturn::Continue,
                Action::SwitchEditMode => {
                    self.state.input_mode = InputMode::Editing;
                    AppReturn::Continue
                }
                Action::SwitchNormalMode => {
                    self.state.input_mode = InputMode::Editing;
                    AppReturn::Continue
                }
                Action::ShowChList => {
                    self.state.set_view_ch_list();
                    AppReturn::Continue
                }
                Action::ShowOpenCh => {
                    self.state.set_view_open_ch();
                    AppReturn::Continue
                }
                Action::Down => {
                    self.state.next_ch();
                    AppReturn::Continue
                }
                Action::Up => {
                    self.state.previous_ch();
                    AppReturn::Continue
                }
            }
        } else {
            warn!("No action accociated to {}", key);

            AppReturn::Continue
        }
    }

    pub async fn handle_edit_key(&mut self, key: Key) -> AppReturn {
        match key {
            Key::Enter => {
                self.state.input_returned =
                    self.state.input_text.drain(..).collect();
                AppReturn::Continue
            }
            Key::Char(c) => {
                self.state.input_text.push(c);
                AppReturn::Continue
            }
            Key::Backspace => {
                self.state.input_text.pop();
                AppReturn::Continue
            }
            Key::Esc => {
                self.state.input_mode = InputMode::Normal;

                AppReturn::Continue
            }
            _ => AppReturn::Continue,
        }
    }

    pub async fn handle_others(&mut self, key: Key) -> AppReturn {
        match key {
            Key::Esc => {
                self.state.input_mode = InputMode::Normal;

                AppReturn::Continue
            }
            _ => AppReturn::Continue,
        }
    }

    /// We could update the app or dispatch event on tick
    pub async fn update_on_tick(&mut self) -> AppReturn {
        // here we just increment a counter
        self.state.incr_tick();
        AppReturn::Continue
    }

    /// Send a network event to the IO thread
    pub async fn dispatch(&mut self, action: IoEvent) {
        // `is_loading` will be set to false again after the async
        // action has finished in io/handler.rs
        self.state.is_loading = true;

        if let Err(e) = self.io_tx.send(action).await {
            self.state.is_loading = false;
            error!("Error from dispatch {}", e);
        };
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    pub(crate) fn get_state(&self) -> &AppState {
        &self.state
    }

    pub(crate) fn get_state_mut(&mut self) -> &mut AppState {
        &mut self.state
    }

    pub fn is_loading(&self) -> bool {
        self.state.is_loading
    }

    pub fn initialized(&mut self) {
        self.actions = vec![
            Action::Quit,
            Action::Sleep,
            Action::SwitchEditMode,
            Action::SwitchNormalMode,
            Action::ShowOpenCh,
            Action::ShowChList,
            Action::Down,
            Action::Up,
        ]
        .into();

        self.state = AppState::initialized()
    }

    pub fn loaded(&mut self) {
        self.state.is_loading = false;
    }

    pub fn slept(&mut self) {
        self.state.incr_sleep();
    }

    pub fn set_some_state(&mut self, data: String) {
        self.state.set_some_state(data);
    }
}
