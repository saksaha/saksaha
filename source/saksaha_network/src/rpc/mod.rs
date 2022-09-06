mod routes;
mod rpc;

#[cfg(test)]
mod tests;

pub(crate) use rpc::*;

pub(crate) type RPCError = Box<dyn std::error::Error + Send + Sync>;
