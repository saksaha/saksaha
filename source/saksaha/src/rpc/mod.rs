mod response;
mod router;
mod routes;
mod rpc;
mod server;

#[cfg(test)]
mod tests;

pub(crate) use rpc::*;

pub(crate) type RPCError = Box<dyn std::error::Error + Send + Sync>;
