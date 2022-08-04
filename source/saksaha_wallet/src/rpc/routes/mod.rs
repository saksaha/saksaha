mod v0;

use hyper_rpc_router::{Handler, Path};
use std::{collections::HashMap, sync::Arc};
pub(crate) use v0::*;

pub(crate) fn get_routes() -> HashMap<&'static str, Handler<Arc<usize>>> {
    let paths: Vec<Path<Arc<usize>>> = vec![];

    let mut map = HashMap::new();
    for p in paths.into_iter() {
        map.insert(p.method, p.handler);
    }

    map
}
