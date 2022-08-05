use crate::wallet::Wallet;
use std::sync::Arc;

pub(crate) struct RouteCtx {
    pub wallet: Arc<Wallet>,
}
