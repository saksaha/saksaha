use super::{AppReturn, AppState, Envelope, View};
use crate::envelope::actions;
use crate::envelope::dispatcher::Dispatch;
use crate::io::InputMode;
use crate::{envelope::Action, inputs::key::Key};
use log::{info, warn};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::RwLockWriteGuard;

impl Envelope {
    pub async fn handle_normal_key<'a>(
        &self,
        key: Key,
        state: RwLockWriteGuard<'a, AppState>,
    ) -> AppReturn {
        let conn_node_port = match self.rpc.node_port {
            Some(p) => p,
            None => 0,
        };

        let conn_wallet_port = match self.rpc.wallet_port {
            Some(p) => p,
            None => 0,
        };

        info!("Run action [{:?}], actions: {:?}", key, self.get_actions());

        if let Some(ref action) = self.get_actions().find(key) {
            // let mut state = self.state.write().await;
            // state.input_text.clear();

            match action {
                Action::Quit => AppReturn::Exit,

                Action::SwitchEditMode => {
                    // state.input_mode = InputMode::Editing;
                    self.dispatch(Action::SwitchEditMode).await;

                    AppReturn::Continue
                }

                Action::SwitchNormalMode => {
                    // state.input_mode = InputMode::Editing;
                    self.dispatch(Action::SwitchNormalMode).await;

                    AppReturn::Continue
                }

                Action::ShowChList => {
                    // let _ = self.get_ch_list().await;
                    // let _ = self.get_ch_list_from_local().await;
                    // state.set_view_ch_list();

                    let dispatcher = self.dispatcher.clone();
                    let dispatch: Dispatch = Box::new(move |action| {
                        let d = dispatcher.clone();
                        Box::pin(async move {
                            d.dispatch(action).await?;
                            Ok::<_, SendError<Action>>(())
                        })
                    });

                    actions::show_ch_list(
                        conn_node_port,
                        dispatch,
                        state,
                        self.dispatcher.get_context().clone(),
                    )
                    .await;

                    // self.dispatch(Action::ShowChList).await;

                    AppReturn::Continue
                }

                Action::ShowOpenCh => {
                    // state.set_view_open_ch();
                    self.dispatch(Action::ShowOpenCh).await;

                    AppReturn::Continue
                }

                Action::ShowChat => {
                    self.dispatch(Action::ShowChat).await;
                    // state.set_view_chat();

                    AppReturn::Continue
                }

                Action::Down => {
                    // state.next_ch();
                    self.dispatch(Action::Down).await;

                    AppReturn::Continue
                }
                Action::Up => {
                    // state.previous_ch();
                    self.dispatch(Action::Up).await;

                    AppReturn::Continue
                }

                Action::RestoreChat => {
                    let dispatcher = self.dispatcher.clone();
                    let dispatch: Dispatch = Box::new(move |action| {
                        let d = dispatcher.clone();
                        Box::pin(async move {
                            d.dispatch(action).await?;
                            Ok::<_, SendError<Action>>(())
                        })
                    });

                    actions::restore_chat(conn_node_port, dispatch, state)
                        .await;

                    // self.dispatch(Action::RestoreChat).await;

                    // self.dispatch(Action::RestoreChatSuccess).await;

                    // match state.view {
                    //     View::Chat => {
                    //         let ch_id = state.selected_ch_id.clone();

                    //         if !ch_id.is_empty() {
                    //             self.get_messages(ch_id.clone()).await;

                    //             log::info!(
                    //                 "Restore all the chats in ch_id: {:?}",
                    //                 ch_id
                    //             );
                    //         }

                    //         return AppReturn::Continue;
                    //     }
                    //     _ => {
                    //         return AppReturn::Continue;
                    //     }
                    // }
                    AppReturn::Continue
                }

                Action::Select => {
                    let dispatcher = self.dispatcher.clone();
                    let dispatch: Dispatch = Box::new(move |action| {
                        let d = dispatcher.clone();
                        Box::pin(async move {
                            d.dispatch(action).await?;
                            Ok::<_, SendError<Action>>(())
                        })
                    });

                    actions::select(conn_node_port, dispatch, state).await;

                    AppReturn::Continue

                    // match state.view {
                    //     View::ChList => {
                    //         state.selected_ch_id = match state
                    //             .ch_list_state
                    //             .selected()
                    //         {
                    //             Some(i) => (state.ch_list[i]).channel.ch_id.clone(),
                    //             None => String::default(),
                    //         };

                    //         log::info!("Ch_Id: {:?}", state.selected_ch_id);

                    //         // self.get_messages(self.state.selected_ch_id.clone())
                    //         //     .await;

                    //         state.set_view_chat();
                    //         return AppReturn::Continue;
                    //     }
                    //     _ => {
                    //         return AppReturn::Continue;
                    //     }
                    // },
                }

                Action::UpdateBalance => {
                    log::info!("UPDATE_BALANCE");
                    let dispatcher = self.dispatcher.clone();
                    let dispatch: Dispatch = Box::new(move |action| {
                        let d = dispatcher.clone();
                        Box::pin(async move {
                            d.dispatch(action).await?;
                            Ok::<_, SendError<Action>>(())
                        })
                    });

                    actions::update_balance(
                        conn_wallet_port,
                        dispatch,
                        state,
                        self.dispatcher.get_context().clone(),
                    )
                    .await;

                    AppReturn::Continue
                }

                // Action::UpdateBalance => {
                //     let my_pk = self.get_credential().acc_addr.clone();

                //     state.set_balance(my_pk).await;
                //     AppReturn::Continue
                // }
                _ => {
                    // Some actions are not mapped with key inputs
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
        mut state: RwLockWriteGuard<'a, AppState>,
    ) -> AppReturn {
        let conn_node_port = match self.rpc.node_port {
            Some(p) => p,
            None => 0,
        };

        let conn_wallet_port = match self.rpc.wallet_port {
            Some(p) => p,
            None => 0,
        };

        match key {
            Key::Enter => {
                // let mut state = self.state.write().await;

                match state.view {
                    View::OpenCh => {
                        let dispatcher = self.dispatcher.clone();
                        let dispatch: Dispatch = Box::new(move |action| {
                            let d = dispatcher.clone();
                            Box::pin(async move {
                                d.dispatch(action).await?;
                                Ok::<_, SendError<Action>>(())
                            })
                        });

                        actions::enter_in_open_ch(
                            conn_wallet_port,
                            dispatch,
                            state,
                            self.dispatcher.get_context().clone(),
                        )
                        .await;

                        // state.input_returned =
                        //     state.input_text.drain(..).collect();

                        // // need to check validity of `self.state.input_returned`
                        // // let pk = self.state.input_returned.clone();

                        // // for dev
                        // {
                        // if let Err(_) = &self
                        //     // .open_ch(&self.get_partner_pk().to_owned())
                        //     .open_ch(&state.input_returned)
                        //     .await
                        //     {
                        //         return AppReturn::Continue;
                        //     }
                        // };
                    }
                    View::Chat => {
                        let dispatcher = self.dispatcher.clone();
                        let dispatch: Dispatch = Box::new(move |action| {
                            let d = dispatcher.clone();
                            Box::pin(async move {
                                d.dispatch(action).await?;
                                Ok::<_, SendError<Action>>(())
                            })
                        });

                        actions::enter_in_chat(
                            conn_node_port,
                            conn_wallet_port,
                            dispatch,
                            state,
                            self.dispatcher.get_context().clone(),
                        )
                        .await;

                        // if state.selected_ch_id != String::default() {
                        //     state.chat_input =
                        //         state.input_text.drain(..).collect();

                        // match self.send_messages(&state.chat_input).await {
                        //         Ok(res) => {
                        //             log::info!(
                        //                 "[send_message] Result: {:?}",
                        //                 res
                        //             );
                        //             AppReturn::Continue
                        //         }
                        //         Err(err) => {
                        //             log::warn!(
                        //                 "[send_message] Error: {:?}",
                        //                 err
                        //             );
                        //             AppReturn::Continue
                        //         }
                        //     };
                        // } else {
                        //     let _trash_bin: String =
                        //         state.input_text.drain(..).collect();

                        //     log::error!(
                        //         "[send_message] You should get the \
                        //         `ch_id` first!"
                        //     );
                        // }

                        // self.get_state_mut()
                        //     .set_input_messages(self.get_state_mut().chat_input.clone());
                    }
                    _ => {}
                }

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
        }
    }
}
