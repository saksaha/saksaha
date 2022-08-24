use crate::credential::Credential;
use std::sync::Arc;

pub(crate) struct DispatcherContext {
    pub credential: Arc<Credential>,
}
