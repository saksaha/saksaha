use super::{AppReturn, Envelope, View};
use crate::envelope::actions::Action;
use crate::inputs::key::Key;
use crate::io::InputMode;
use log::{debug, warn};

impl Envelope {
    pub async fn handle_normal_key(&mut self, key: Key) -> AppReturn {
        if let Some(&action) = self.get_actions().find(key) {
            debug!("Run action [{:?}]", action);
            self.get_state_mut().input_text.clear();

            match action {
                Action::Quit => AppReturn::Exit,

                Action::SwitchEditMode => {
                    self.get_state_mut().input_mode = InputMode::Editing;
                    AppReturn::Continue
                }

                Action::SwitchNormalMode => {
                    self.get_state_mut().input_mode = InputMode::Editing;
                    AppReturn::Continue
                }

                Action::ShowChList => {
                    let _ = self.get_ch_list().await;
                    // let _ = self.get_ch_list_from_local().await;
                    self.get_state_mut().set_view_ch_list();
                    AppReturn::Continue
                }

                Action::ShowOpenCh => {
                    self.get_state_mut().set_view_open_ch();
                    AppReturn::Continue
                }

                Action::ShowChat => {
                    self.get_state_mut().set_view_chat();
                    AppReturn::Continue
                }

                Action::Down => {
                    self.get_state_mut().next_ch();
                    AppReturn::Continue
                }

                Action::Up => {
                    self.get_state_mut().previous_ch();
                    AppReturn::Continue
                }

                Action::RestoreChat => match self.get_state().view {
                    View::Chat => {
                        let ch_id = self.get_state().selected_ch_id.clone();

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
                Action::Select => match self.get_state().view {
                    View::ChList => {
                        self.get_state_mut().selected_ch_id =
                            match self.get_state().ch_list_state.selected() {
                                Some(i) => (self.get_state().ch_list[i])
                                    .channel
                                    .ch_id
                                    .clone(),
                                None => String::default(),
                            };

                        log::info!(
                            "Ch_Id: {:?}",
                            self.get_state().selected_ch_id
                        );
                        // self.get_messages(self.state.selected_ch_id.clone())
                        //     .await;
                        self.get_state_mut().set_view_chat();
                        return AppReturn::Continue;
                    }
                    _ => {
                        return AppReturn::Continue;
                    }
                },

                Action::UpdateBalance => {
                    let my_pk = self.get_credential().acc_addr.clone();

                    self.get_state_mut().set_balance(my_pk).await;
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
                match self.get_state().view {
                    View::OpenCh => {
                        self.get_state_mut().input_returned =
                            self.get_state_mut().input_text.drain(..).collect();

                        // need to check validity of `self.state.input_returned`
                        // let pk = self.state.input_returned.clone();

                        // for dev
                        {
                            if let Err(_) = self
                                // .open_ch(&self.get_partner_pk().to_owned())
                                .open_ch(
                                    &self.get_state().input_returned.clone(),
                                )
                                .await
                            {
                                return AppReturn::Continue;
                            }
                        };
                    }
                    View::Chat => {
                        if self.get_state().selected_ch_id != String::default()
                        {
                            self.get_state_mut().chat_input = self
                                .get_state_mut()
                                .input_text
                                .drain(..)
                                .collect();

                            match self
                                .send_messages(&self.get_state().chat_input)
                                .await
                            {
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
                            let _trash_bin: String = self
                                .get_state_mut()
                                .input_text
                                .drain(..)
                                .collect();

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
                self.get_state_mut().input_text.push(c);
                AppReturn::Continue
            }
            Key::Backspace => {
                self.get_state_mut().input_text.pop();
                AppReturn::Continue
            }
            Key::Esc => {
                self.get_state_mut().input_mode = InputMode::Normal;

                AppReturn::Continue
            }
            _ => AppReturn::Continue,
        }
    }

    pub async fn handle_others(&mut self, key: Key) -> AppReturn {
        match key {
            Key::Esc => {
                self.get_state_mut().input_mode = InputMode::Normal;

                AppReturn::Continue
            }
            _ => AppReturn::Continue,
        }
    }
}
