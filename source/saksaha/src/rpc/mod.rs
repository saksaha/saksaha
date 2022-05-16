use hyper::server::conn::AddrIncoming;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use log::{debug, info};
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct RPC {}

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}

impl RPC {
    pub async fn init() -> Result<(), Box<dyn std::error::Error + Send + Sync>>
    {
        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(hello))
        });

        let addr = ([127, 0, 0, 1], 3000).into();

        let server = Server::bind(&addr).serve(make_svc);

        println!("Listening on http://{}", addr);

        server.await?;

        Ok(())
    }
}
