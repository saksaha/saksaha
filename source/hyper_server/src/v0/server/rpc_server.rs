use super::{Middleware, StateMachine};
use crate::RPCServerError;
use hyper::{server::conn::AddrIncoming, service, Body, Response, Server};
use sak_logger::{debug, error};
use std::{convert::Infallible, sync::Arc};
use tokio::net::TcpListener;

pub struct RPCServer {}

impl RPCServer {
    pub async fn run<C>(
        self,
        rpc_socket: TcpListener,
        ctx: C,
        middlewares: Vec<Middleware<C>>,
    ) -> Result<(), RPCServerError>
    where
        C: Clone + Send + Sync + 'static,
    {
        let addr_incoming = match AddrIncoming::from_listener(rpc_socket) {
            Ok(a) => a,
            Err(err) => {
                return Err(format!(
                    "Error initializing Addr Incoming, err: {}",
                    err
                )
                .into());
            }
        };

        let middlewares = Arc::new(middlewares);

        let make_svc = service::make_service_fn(move |_conn| {
            let ctx = ctx.clone();

            let state_machine = {
                let m = StateMachine {
                    middlewares: middlewares.clone(),
                };

                Arc::new(m)
            };

            async move {
                Ok::<_, Infallible>(service::service_fn(move |req| {
                    // debug!(
                    //     "rpc, method: {}, uri: {}",
                    //     req.method(),
                    //     req.uri().path()
                    // );

                    let ctx_clone = ctx.clone();
                    let state_machine_clone = state_machine.clone();
                    let resp: Response<Body> = Response::default();

                    async move {
                        let res =
                            state_machine_clone.run(req, resp, ctx_clone).await;

                        res
                    }
                }))
            }
        });

        let server = Server::builder(addr_incoming).serve(make_svc);

        if let Err(err) = server.await {
            error!("Error running rpc server, err: {}", err);
        }

        Ok(())
    }
}
