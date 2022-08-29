use super::dispatcher::Dispatcher;
use super::reducer::DispatcherContext;
use super::state::AppState;
use crate::credential::Credential;
use crate::db::EnvelopeDB;
use crate::{envelope::actions::Action, ENVELOPE_CTR_ADDR};
use crate::{wallet_sdk, EnvelopeError, RPCConfig};
use envelope_contract::request_type::OPEN_CH;
use envelope_contract::{Channel, OpenChParams};
use sak_contract_std::{CtrCallType, CtrRequest};
use sak_crypto::{PublicKey, SakKey, ToEncodedPoint};
use std::sync::Arc;
use tokio::sync::RwLock;
use type_extension::{U8Arr32, U8Array};

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

// pub(crate) type ActionCreator = Box<
//     dyn Fn(
//             Dispatch,
//             Arc<RwLock<AppState>>,
//             // RwLockWriteGuard<AppState>,
//         ) -> Pin<
//             Box<dyn Future<Output = Result<(), EnvelopeError>> + Send + Sync>,
//         > + Send
//         + Sync,
// >;

pub(crate) struct Envelope {
    // pub(super) io_tx: mpsc::Sender<IoEvent>,
    pub(crate) dispatcher: Arc<Dispatcher>,
    // pub(super) actions: Actions,
    pub(super) state: Arc<RwLock<AppState>>,
    pub(super) db: EnvelopeDB,
    pub(super) credential: Arc<Credential>,
    pub(super) partner_credential: Arc<Credential>,
    pub wallet_endpoint: String,
    pub saksaha_endpoint: String,
}

impl Envelope {
    pub(crate) async fn init(
        // io_tx: mpsc::Sender<IoEvent>,
        // rpc: RPCConfig,
        credential: Arc<Credential>,
        partner_credential: Arc<Credential>,
        wallet_endpoint: String,
        saksaha_endpoint: String,
    ) -> Result<Self, EnvelopeError> {
        // let actions = {
        //     Actions(vec![
        //         Action::Quit,
        //         Action::SwitchEditMode,
        //         Action::SwitchNormalMode,
        //         Action::ShowOpenCh,
        //         Action::ShowChList,
        //         Action::ShowChat,
        //         Action::DownCh,
        //         Action::UpCh,
        //         Action::DownChat,
        //         Action::UpChat,
        //         Action::PageUpChat,
        //         Action::UpdateBalance,
        //         Action::Select,
        //         Action::RestoreChat,
        //     ])
        // };

        let state = {
            let s = AppState::default();

            Arc::new(RwLock::new(s))
        };

        let db = EnvelopeDB::init(&credential.acc_addr).await?;

        let dispatcher = {
            let ctx = {
                let c = DispatcherContext {
                    credential: credential.clone(),
                };
                Arc::new(c)
            };

            let d = Dispatcher::new(state.clone(), ctx)?;
            Arc::new(d)
        };

        let dispatcher_clone = dispatcher.clone();
        tokio::spawn(async move {
            dispatcher_clone.run().await;
        });

        Ok(Self {
            // io_tx,
            dispatcher,
            // actions,
            state,
            db,
            credential,
            partner_credential,
            wallet_endpoint,
            saksaha_endpoint,
        })
    }

    pub async fn update_on_tick(&self) -> AppReturn {
        AppReturn::Continue
    }

    // pub fn get_actions(&self) -> &Actions {
    //     &self.actions
    // }

    pub fn get_db(&self) -> &EnvelopeDB {
        &self.db
    }

    pub fn get_state(&self) -> &Arc<RwLock<AppState>> {
        &self.state
    }

    pub fn get_credential(&self) -> &Credential {
        &self.credential
    }

    pub async fn dispatch(&self, action: Action) -> Result<(), EnvelopeError> {
        self.dispatcher.dispatch(action).await?;

        Ok(())
    }
}
