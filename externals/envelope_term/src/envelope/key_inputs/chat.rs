use crate::envelope::dispatcher::Dispatch;
use crate::envelope::{actions, AppReturn, AppState};
use crate::io::InputMode;
use crate::Envelope;
use crate::{envelope::Action, inputs::key::Key};
use log::warn;
use tokio::sync::mpsc::error::SendError;
use tokio::sync::RwLockWriteGuard;

impl Envelope {
    pub async fn handle_key_input_in_chat<'a>(
        &self,
        key: Key,
        mut state: RwLockWriteGuard<'a, AppState>,
    ) -> AppReturn {
        match state.input_mode {
            InputMode::Normal => match key {
                Key::Ctrl('c') => AppReturn::Exit,
                Key::Char('q') => AppReturn::Exit,
                Key::Char('i') => {
                    self.dispatch(Action::SwitchEditMode).await;

                    AppReturn::Continue
                }
                Key::Esc => {
                    self.dispatch(Action::SwitchNormalMode).await;

                    AppReturn::Continue
                }
                Key::Char('1') => {
                    let dispatcher = self.dispatcher.clone();
                    let dispatch: Dispatch = Box::new(move |action| {
                        let d = dispatcher.clone();
                        Box::pin(async move {
                            d.dispatch(action).await?;
                            Ok::<_, SendError<Action>>(())
                        })
                    });

                    actions::show_ch_list(
                        self.saksaha_endpoint.clone(),
                        dispatch,
                        state,
                        self.dispatcher.get_context().clone(),
                    )
                    .await;

                    AppReturn::Continue
                }
                Key::Char('2') => {
                    self.dispatch(Action::ShowOpenCh).await;

                    AppReturn::Continue
                }
                Key::Down => {
                    self.dispatch(Action::DownChat).await;

                    AppReturn::Continue
                }
                Key::Up => {
                    self.dispatch(Action::UpChat).await;

                    AppReturn::Continue
                }
                Key::Char('R') => {
                    let dispatcher = self.dispatcher.clone();
                    let dispatch: Dispatch = Box::new(move |action| {
                        let d = dispatcher.clone();
                        Box::pin(async move {
                            d.dispatch(action).await?;
                            Ok::<_, SendError<Action>>(())
                        })
                    });

                    actions::restore_chat(
                        self.saksaha_endpoint.clone(),
                        dispatch,
                        state,
                    )
                    .await;

                    AppReturn::Continue
                }
                Key::Char('$') => {
                    let dispatcher = self.dispatcher.clone();
                    let dispatch: Dispatch = Box::new(move |action| {
                        let d = dispatcher.clone();
                        Box::pin(async move {
                            d.dispatch(action).await?;
                            Ok::<_, SendError<Action>>(())
                        })
                    });

                    actions::update_balance(
                        self.wallet_endpoint.clone(),
                        dispatch,
                        state,
                        self.dispatcher.get_context().clone(),
                    )
                    .await;

                    AppReturn::Continue
                }
                Key::Enter => {
                    let dispatcher = self.dispatcher.clone();
                    let dispatch: Dispatch = Box::new(move |action| {
                        let d = dispatcher.clone();
                        Box::pin(async move {
                            d.dispatch(action).await?;
                            Ok::<_, SendError<Action>>(())
                        })
                    });

                    actions::select(
                        self.saksaha_endpoint.clone(),
                        dispatch,
                        state,
                    )
                    .await;

                    AppReturn::Continue
                }
                _ => {
                    warn!("No action accociated to {}", key);

                    AppReturn::Continue
                }
            },
            InputMode::Editing => match key {
                Key::Enter => {
                    let dispatcher = self.dispatcher.clone();
                    let dispatch: Dispatch = Box::new(move |action| {
                        let d = dispatcher.clone();
                        Box::pin(async move {
                            d.dispatch(action).await?;
                            Ok::<_, SendError<Action>>(())
                        })
                    });

                    actions::enter_in_chat(
                        self.saksaha_endpoint.clone(),
                        self.wallet_endpoint.clone(),
                        dispatch,
                        state,
                        self.dispatcher.get_context().clone(),
                    )
                    .await;

                    AppReturn::Continue
                }
                Key::Char(c) => {
                    state.input_text.push(c);

                    AppReturn::Continue
                }
                Key::Backspace => {
                    state.input_text.pop();

                    AppReturn::Continue
                }
                Key::Esc => {
                    state.input_mode = InputMode::Normal;

                    AppReturn::Continue
                }
                _ => AppReturn::Continue,
            },
        }
    }
}
