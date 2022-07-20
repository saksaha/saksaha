use super::v0;
use crate::rpc::{server::State, Route, RouteMap};
use hyper::Method;
use std::{collections::HashMap, sync::Arc};

// pub(in crate::rpc) fn get_route_map() -> RouteMap<Arc<State>> {
//     HashMap::from([
//         (
//             "/apis/v0/get_transaction",
//             Route {
//                 method: Method::POST,
//                 handler: Box::new(|req, state| {
//                     Box::pin(v0::get_transaction(req, state))
//                 }),
//             },
//         ),
//         (
//             "/apis/v0/send_transaction",
//             Route {
//                 method: Method::POST,
//                 handler: Box::new(|req, state| {
//                     Box::pin(v0::send_transaction(req, state))
//                 }),
//             },
//         ),
//         (
//             "/apis/v0/get_status",
//             Route {
//                 method: Method::POST,
//                 handler: Box::new(|req, state| {
//                     Box::pin(v0::get_status(req, state))
//                 }),
//             },
//         ),
//         (
//             "/apis/v0/get_block",
//             Route {
//                 method: Method::POST,
//                 handler: Box::new(|req, sys_handle| {
//                     Box::pin(v0::get_block(req, sys_handle))
//                 }),
//             },
//         ),
//     ])
// }
