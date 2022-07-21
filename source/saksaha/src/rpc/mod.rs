mod response;
mod route_map;
mod router;
mod routes;
mod rpc;

#[cfg(test)]
mod tests;

pub(in crate::rpc) use response::*;
pub(in crate::rpc) use route_map::*;
pub(crate) use rpc::*;
pub(crate) type RPCError = Box<dyn std::error::Error + Send + Sync>;
