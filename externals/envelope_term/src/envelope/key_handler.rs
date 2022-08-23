use super::{AppReturn, AppState, Envelope, View};
use crate::envelope::actions::Action;
use crate::inputs::key::Key;
use crate::io::InputMode;
use log::{debug, info, warn};
use tokio::sync::RwLockWriteGuard;

impl Envelope {
    pub async fn handle_normal_key<'a>(
        &self,
        key: Key,
        //
        mut state: RwLockWriteGuard<'a, AppState>,
    ) -> AppReturn {
        info!("Run action [{:?}], actions: {:?}", key, self.get_actions());

        if let Some(&action) = self.get_actions().find(key) {
            // let mut state = self.state.write().await;
            state.input_text.clear();

            match action {
                Action::Quit => AppReturn::Exit,

                Action::SwitchEditMode => {
                    state.input_mode = InputMode::Editing;
                    AppReturn::Continue
                }

                Action::SwitchNormalMode => {
                    state.input_mode = InputMode::Editing;
                    AppReturn::Continue
                }

                Action::ShowChList => {
                    let _ = self.get_ch_list().await;
                    // let _ = self.get_ch_list_from_local().await;
                    state.set_view_ch_list();
                    AppReturn::Continue
                }

                Action::ShowOpenCh => {
                    state.set_view_open_ch();
                    AppReturn::Continue
                }

                Action::ShowChat => {
                    state.set_view_chat();
                    AppReturn::Continue
                }

                Action::Down => {
                    state.next_ch();
                    AppReturn::Continue
                }

                Action::Up => {
                    state.previous_ch();
                    AppReturn::Continue
                }

                Action::RestoreChat => match state.view {
                    View::Chat => {
                        let ch_id = state.selected_ch_id.clone();

                        if !ch_id.is_empty() {
                            self.get_messages(ch_id.clone()).await;

                            log::info!(
                                "Restore all the chats in ch_id: {:?}",
                                ch_id
                            );
                        }

                        return AppReturn::Continue;
                    }
                    _ => {
                        return AppReturn::Continue;
                    }
                },
                Action::Select => match state.view {
                    View::ChList => {
                        state.selected_ch_id = match state
                            .ch_list_state
                            .selected()
                        {
                            Some(i) => (state.ch_list[i]).channel.ch_id.clone(),
                            None => String::default(),
                        };

                        log::info!("Ch_Id: {:?}", state.selected_ch_id);

                        // self.get_messages(self.state.selected_ch_id.clone())
                        //     .await;

                        state.set_view_chat();
                        return AppReturn::Continue;
                    }
                    _ => {
                        return AppReturn::Continue;
                    }
                },

                Action::UpdateBalance => {
                    let my_pk = self.get_credential().acc_addr.clone();

                    state.set_balance(my_pk).await;
                    AppReturn::Continue
                }
            }
        } else {
            warn!("No action accociated to {}", key);

            AppReturn::Continue
        }
    }

    pub async fn handle_edit_key<'a>(
        &self,
        key: Key,
        //
        state: RwLockWriteGuard<'a, AppState>,
    ) -> AppReturn {
        match key {
            Key::Enter => {
                let mut state = self.state.write().await;

                match state.view {
                    View::OpenCh => {
                        state.input_returned =
                            state.input_text.drain(..).collect();

                        // need to check validity of `self.state.input_returned`
                        // let pk = self.state.input_returned.clone();

                        // for dev
                        {
                            if let Err(_) = &self
                                // .open_ch(&self.get_partner_pk().to_owned())
                                .open_ch(&state.input_returned)
                                .await
                            {
                                return AppReturn::Continue;
                            }
                        };
                    }
                    View::Chat => {
                        if state.selected_ch_id != String::default() {
                            state.chat_input =
                                state.input_text.drain(..).collect();

                            match self.send_messages(&state.chat_input).await {
                                Ok(res) => {
                                    log::info!(
                                        "[send_message] Result: {:?}",
                                        res
                                    );
                                    AppReturn::Continue
                                }
                                Err(err) => {
                                    log::warn!(
                                        "[send_message] Error: {:?}",
                                        err
                                    );
                                    AppReturn::Continue
                                }
                            };
                        } else {
                            let _trash_bin: String =
                                state.input_text.drain(..).collect();

                            log::error!(
                                "[send_message] You should get the \
                                `ch_id` first!"
                            );
                        }

                        // self.get_state_mut()
                        //     .set_input_messages(self.get_state_mut().chat_input.clone());
                    }
                    _ => {}
                }

                AppReturn::Continue
            }
            Key::Char(c) => {
                let mut state = self.state.write().await;

                state.input_text.push(c);
                AppReturn::Continue
            }
            Key::Backspace => {
                let mut state = self.state.write().await;

                state.input_text.pop();
                AppReturn::Continue
            }
            Key::Esc => {
                let mut state = self.state.write().await;

                state.input_mode = InputMode::Normal;

                AppReturn::Continue
            }
            _ => AppReturn::Continue,
        }
    }

    pub async fn handle_others(&mut self, key: Key) -> AppReturn {
        match key {
            Key::Esc => {
                let mut state = self.state.write().await;

                state.input_mode = InputMode::Normal;

                AppReturn::Continue
            }
            _ => AppReturn::Continue,
        }
    }
}
