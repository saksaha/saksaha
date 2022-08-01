pub(in crate::rpc) mod v0;

use super::router::{Handler, Path};
use crate::SystemHandle;
use std::{collections::HashMap, sync::Arc};

pub(in crate::rpc) fn get_routes(
) -> HashMap<&'static str, Handler<Arc<SystemHandle>>> {
    let paths: Vec<Path<Arc<SystemHandle>>> = vec![
        Path {
            method: "send_mint_tx",
            handler: Box::new(|route_state, params, sys_handle| {
                Box::pin(v0::send_mint_tx(route_state, params, sys_handle))
            }),
        },
        Path {
            method: "send_pour_tx",
            handler: Box::new(|route_state, params, sys_handle| {
                Box::pin(v0::send_pour_tx(route_state, params, sys_handle))
            }),
        },
        Path {
            method: "get_status",
            handler: Box::new(|route_state, params, sys_handle| {
                Box::pin(v0::get_status(route_state, params, sys_handle))
            }),
        },
        Path {
            method: "get_tx",
            handler: Box::new(|route_state, params, sys_handle| {
                Box::pin(v0::get_tx(route_state, params, sys_handle))
            }),
        },
        Path {
            method: "get_block",
            handler: Box::new(|route_state, params, sys_handle| {
                Box::pin(v0::get_block(route_state, params, sys_handle))
            }),
        },
        Path {
            method: "get_block_list",
            handler: Box::new(|route_state, params, sys_handle| {
                Box::pin(v0::get_block_list(route_state, params, sys_handle))
            }),
        },
        Path {
            method: "query_ctr",
            handler: Box::new(|route_state, params, sys_handle| {
                Box::pin(v0::query_ctr(route_state, params, sys_handle))
            }),
        },
    ];

    let mut map = HashMap::new();
    for p in paths.into_iter() {
        map.insert(p.method, p.handler);
    }

    map
}
