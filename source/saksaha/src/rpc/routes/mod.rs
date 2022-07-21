use super::route_map::{Handler, Path};
use std::collections::HashMap;
pub(super) mod v0;

pub(in crate::rpc) fn get_routes() -> HashMap<&'static str, Handler> {
    let paths: Vec<Path> = vec![
        Path {
            url: "/apis/v0/send_mint_tx",
            handler: Box::new(|req, sys_handle| {
                Box::pin(v0::send_mint_tx(req, sys_handle))
            }),
        },
        Path {
            url: "/apis/v0/send_pour_tx",
            handler: Box::new(|req, sys_handle| {
                Box::pin(v0::send_pour_tx(req, sys_handle))
            }),
        },
        Path {
            url: "/apis/v0/get_status",
            handler: Box::new(|req, sys_handle| {
                Box::pin(v0::get_status(req, sys_handle))
            }),
        },
        Path {
            url: "/apis/v0/get_transaction",
            handler: Box::new(|req, sys_handle| {
                Box::pin(v0::get_transaction(req, sys_handle))
            }),
        },
        Path {
            url: "/apis/v0/get_block",
            handler: Box::new(|req, sys_handle| {
                Box::pin(v0::get_block(req, sys_handle))
            }),
        },
        Path {
            url: "/apis/v0/call_contract",
            handler: Box::new(|req, sys_handle| {
                Box::pin(v0::query_ctr(req, sys_handle))
            }),
        },
    ];

    let mut map = HashMap::new();
    for p in paths.into_iter() {
        map.insert(p.url, p.handler);
    }

    map
}
