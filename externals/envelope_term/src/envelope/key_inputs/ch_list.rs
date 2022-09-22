use crate::envelope::dispatcher::{self, Dispatch, Dispatcher};
use crate::envelope::{actions, AppReturn, AppState, View};
use crate::Envelope;
use crate::{envelope::Action, inputs::key::Key};
use tokio::sync::mpsc::error::SendError;
use tokio::sync::RwLockWriteGuard;

impl Envelope {
    pub async fn handle_key_input_in_ch_list<'a>(
        &self,
        key: Key,
        state: RwLockWriteGuard<'a, AppState>,
    ) -> AppReturn {
        let _ = match key {
            Key::Ctrl('c') => return AppReturn::Exit,
            Key::Char('q') => return AppReturn::Exit,
            Key::Char('1') => self.dispatch(Action::ShowChList).await,
            Key::Char('2') => self.dispatch(Action::ShowOpenCh).await,
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
                .await
            }
            Key::Char('G') => {
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
                .await
            }

            Key::Up => self.dispatch(Action::UpCh).await,
            Key::Down => self.dispatch(Action::DownCh).await,
            Key::Enter => {
                let dispatcher = self.dispatcher.clone();
                let dispatch: Dispatch = Box::new(move |action| {
                    let d = dispatcher.clone();
                    Box::pin(async move {
                        d.dispatch(action).await?;
                        Ok::<_, SendError<Action>>(())
                    })
                });

                actions::select(self.saksaha_endpoint.clone(), dispatch, state).await
            }
            _ => {
                return AppReturn::Continue;
            }
        };

        AppReturn::Continue
    }
}
