mod response;
mod router;
mod routes;
mod rpc;
mod server;

mod router2;
mod rpc2;

#[cfg(test)]
mod tests;

pub(crate) use rpc::*;
pub(crate) use rpc2::*;

pub(crate) type RPCError = Box<dyn std::error::Error + Send + Sync>;
