pub(crate) mod v0;

use super::ctx::RouteCtx;
use hyper_rpc_router::{Handler, Path};
use std::{collections::HashMap, sync::Arc};

pub(crate) fn get_routes() -> HashMap<&'static str, Handler<Arc<RouteCtx>>> {
    let paths: Vec<Path<Arc<RouteCtx>>> = vec![
        Path {
            method: "send_tx",
            handler: Box::new(|route_state, params, ctx| {
                Box::pin(v0::send_tx(route_state, params, ctx))
            }),
        },
        Path {
            method: "get_balance",
            handler: Box::new(|route_state, params, ctx| {
                Box::pin(v0::get_balance(route_state, params, ctx))
            }),
        },
    ];

    let mut map = HashMap::new();
    for p in paths.into_iter() {
        map.insert(p.method, p.handler);
    }

    map
}
