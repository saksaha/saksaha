use crate::envelope::dispatcher::{Dispatch, Dispatcher};
use crate::envelope::{actions, AppReturn, AppState, View};
use crate::io::InputMode;
use crate::Envelope;
use crate::{envelope::Action, inputs::key::Key};
use log::{info, warn};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::RwLockWriteGuard;

//         Action::Quit => &[Key::Ctrl('c'), Key::Char('q')],
//         Action::SwitchEditMode => &[Key::Char('i')],
//         Action::SwitchNormalMode => &[Key::Esc],
//         Action::ShowChList => &[Key::Char('1')],
//         Action::ShowOpenCh => &[Key::Char('2')],
//         Action::ShowChat => &[Key::Char('3')],
//         Action::DownCh => &[Key::Down],
//         Action::UpCh => &[Key::Up],
//         Action::DownChat => &[Key::Down],
//         Action::UpChat => &[Key::Up],
//         Action::PageUpChat => &[Key::PageUp],
//         Action::RestoreChat => &[Key::Char('R')],
//         Action::UpdateBalance => &[Key::Char('$')],
//         Action::UpdateBalanceSuccess(_) => &[],
//         Action::Select => &[Key::Enter],

//         Action::Initialize => &[],
//         Action::GetChList(_) => &[],
//         Action::GetMessages(_) => &[],

impl Envelope {
    pub async fn handle_key_input<'a>(
        &self,
        key: Key,
        state: RwLockWriteGuard<'a, AppState>,
    ) -> AppReturn {
        match state.view {
            View::Chat => self.handle_key_input_in_chat(key, state).await,
            View::ChList => self.handle_key_input_in_ch_list(key, state).await,
            View::OpenCh => self.handle_key_input_in_open_ch(key, state).await,
            _ => AppReturn::Continue,
        }
    }

    // pub async fn handle_normal_key<'a>(
    //     &self,
    //     key: Key,
    //     state: RwLockWriteGuard<'a, AppState>,
    // ) -> AppReturn {
    //     // info!("Run action [{:?}], actions: {:?}", key, self.get_actions());

    //     match key {
    //         Key::Char('c') => {}
    //         Key::Char('q') => {}
    //         _ => {}
    //     };

    //     if let Some(ref action) = self.get_actions().find(key) {
    //         match action {
    //             Action::Quit => AppReturn::Exit,

    //             Action::SwitchEditMode => {
    //                 self.dispatch(Action::SwitchEditMode).await;

    //                 AppReturn::Continue
    //             }

    //             Action::SwitchNormalMode => {
    //                 self.dispatch(Action::SwitchNormalMode).await;

    //                 AppReturn::Continue
    //             }

    //             Action::ShowChList => {
    //                 let dispatcher = self.dispatcher.clone();
    //                 let dispatch: Dispatch = Box::new(move |action| {
    //                     let d = dispatcher.clone();
    //                     Box::pin(async move {
    //                         d.dispatch(action).await?;
    //                         Ok::<_, SendError<Action>>(())
    //                     })
    //                 });

    //                 actions::show_ch_list(
    //                     self.saksaha_endpoint.clone(),
    //                     dispatch,
    //                     state,
    //                     self.dispatcher.get_context().clone(),
    //                 )
    //                 .await;

    //                 AppReturn::Continue
    //             }

    //             Action::ShowOpenCh => {
    //                 self.dispatch(Action::ShowOpenCh).await;

    //                 AppReturn::Continue
    //             }

    //             Action::ShowChat => {
    //                 self.dispatch(Action::ShowChat).await;

    //                 AppReturn::Continue
    //             }

    //             Action::DownCh => {
    //                 if state.view == View::ChList {
    //                     self.dispatch(Action::DownCh).await;
    //                 } else if state.view == View::Chat {
    //                     self.dispatch(Action::DownChat).await;
    //                 }

    //                 AppReturn::Continue
    //             }
    //             Action::UpCh => {
    //                 if state.view == View::ChList {
    //                     self.dispatch(Action::UpCh).await;
    //                 } else if state.view == View::Chat {
    //                     self.dispatch(Action::UpChat).await;
    //                 }

    //                 AppReturn::Continue
    //             }

    //             // Action::DownChat => {
    //             //     self.dispatch(Action::DownChat).await;

    //             //     AppReturn::Continue
    //             // }

    //             // Action::UpChat => {
    //             //     self.dispatch(Action::UpChat).await;

    //             //     AppReturn::Continue
    //             // }
    //             Action::PageUpChat => {
    //                 self.dispatch(Action::PageUpChat).await;

    //                 AppReturn::Continue
    //             }

    //             Action::RestoreChat => {
    //                 let dispatcher = self.dispatcher.clone();
    //                 let dispatch: Dispatch = Box::new(move |action| {
    //                     let d = dispatcher.clone();
    //                     Box::pin(async move {
    //                         d.dispatch(action).await?;
    //                         Ok::<_, SendError<Action>>(())
    //                     })
    //                 });

    //                 actions::restore_chat(
    //                     self.saksaha_endpoint.clone(),
    //                     dispatch,
    //                     state,
    //                 )
    //                 .await;

    //                 AppReturn::Continue
    //             }

    //             Action::Select => {
    //                 let dispatcher = self.dispatcher.clone();
    //                 let dispatch: Dispatch = Box::new(move |action| {
    //                     let d = dispatcher.clone();
    //                     Box::pin(async move {
    //                         d.dispatch(action).await?;
    //                         Ok::<_, SendError<Action>>(())
    //                     })
    //                 });

    //                 actions::select(
    //                     self.saksaha_endpoint.clone(),
    //                     dispatch,
    //                     state,
    //                 )
    //                 .await;

    //                 AppReturn::Continue
    //             }

    //             Action::UpdateBalance => {
    //                 log::info!("UPDATE_BALANCE");
    //                 let dispatcher = self.dispatcher.clone();
    //                 let dispatch: Dispatch = Box::new(move |action| {
    //                     let d = dispatcher.clone();
    //                     Box::pin(async move {
    //                         d.dispatch(action).await?;
    //                         Ok::<_, SendError<Action>>(())
    //                     })
    //                 });

    //                 actions::update_balance(
    //                     self.wallet_endpoint.clone(),
    //                     dispatch,
    //                     state,
    //                     self.dispatcher.get_context().clone(),
    //                 )
    //                 .await;

    //                 AppReturn::Continue
    //             }

    //             _ => {
    //                 // Some actions are not mapped with key inputs
    //                 AppReturn::Continue
    //             }
    //         }
    //     } else {
    //         warn!("No action accociated to {}", key);

    //         AppReturn::Continue
    //     }
    // }

    // pub async fn handle_edit_key<'a>(
    //     &self,
    //     key: Key,
    //     mut state: RwLockWriteGuard<'a, AppState>,
    // ) -> AppReturn {
    //     match key {
    //         Key::Enter => {
    //             match state.view {
    //                 View::OpenCh => {
    //                     let dispatcher = self.dispatcher.clone();
    //                     let dispatch: Dispatch = Box::new(move |action| {
    //                         let d = dispatcher.clone();
    //                         Box::pin(async move {
    //                             d.dispatch(action).await?;
    //                             Ok::<_, SendError<Action>>(())
    //                         })
    //                     });

    //                     actions::enter_in_open_ch(
    //                         self.wallet_endpoint.clone(),
    //                         dispatch,
    //                         state,
    //                         self.dispatcher.get_context().clone(),
    //                     )
    //                     .await;

    //                     // state.input_returned =
    //                     //     state.input_text.drain(..).collect();

    //                     // // need to check validity of `self.state.input_returned`
    //                     // // let pk = self.state.input_returned.clone();

    //                     // // for dev
    //                     // {
    //                     // if let Err(_) = &self
    //                     //     // .open_ch(&self.get_partner_pk().to_owned())
    //                     //     .open_ch(&state.input_returned)
    //                     //     .await
    //                     //     {
    //                     //         return AppReturn::Continue;
    //                     //     }
    //                     // };
    //                 }
    //                 View::Chat => {
    //                     let dispatcher = self.dispatcher.clone();
    //                     let dispatch: Dispatch = Box::new(move |action| {
    //                         let d = dispatcher.clone();
    //                         Box::pin(async move {
    //                             d.dispatch(action).await?;
    //                             Ok::<_, SendError<Action>>(())
    //                         })
    //                     });

    //                     actions::enter_in_chat(
    //                         self.saksaha_endpoint.clone(),
    //                         self.wallet_endpoint.clone(),
    //                         dispatch,
    //                         state,
    //                         self.dispatcher.get_context().clone(),
    //                     )
    //                     .await;

    //                     // if state.selected_ch_id != String::default() {
    //                     //     state.chat_input =
    //                     //         state.input_text.drain(..).collect();

    //                     // match self.send_messages(&state.chat_input).await {
    //                     //         Ok(res) => {
    //                     //             log::info!(
    //                     //                 "[send_message] Result: {:?}",
    //                     //                 res
    //                     //             );
    //                     //             AppReturn::Continue
    //                     //         }
    //                     //         Err(err) => {
    //                     //             log::warn!(
    //                     //                 "[send_message] Error: {:?}",
    //                     //                 err
    //                     //             );
    //                     //             AppReturn::Continue
    //                     //         }
    //                     //     };
    //                     // } else {
    //                     //     let _trash_bin: String =
    //                     //         state.input_text.drain(..).collect();

    //                     //     log::error!(
    //                     //         "[send_message] You should get the \
    //                     //         `ch_id` first!"
    //                     //     );
    //                     // }

    //                     // self.get_state_mut()
    //                     //     .set_input_messages(self.get_state_mut().chat_input.clone());
    //                 }
    //                 _ => {}
    //             }

    //             AppReturn::Continue
    //         }
    //         Key::Char(c) => {
    //             state.input_text.push(c);

    //             AppReturn::Continue
    //         }
    //         Key::Backspace => {
    //             state.input_text.pop();

    //             AppReturn::Continue
    //         }
    //         Key::Esc => {
    //             state.input_mode = InputMode::Normal;

    //             AppReturn::Continue
    //         }
    //         _ => AppReturn::Continue,
    //     }
    // }
}
