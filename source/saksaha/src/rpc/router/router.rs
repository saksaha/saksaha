use crate::rpc::router::JsonRequest;

use super::{utils, Handler};
use futures::Future;
use hyper::{Body, Request, Response};
use log::debug;
use std::{collections::HashMap, pin::Pin, sync::Arc};

pub(crate) struct Router<C> {
    route_map: Arc<HashMap<&'static str, Handler<C>>>,
}

impl<C> Router<C>
where
    C: Send + Sync + 'static,
{
    pub fn new(route_map: Arc<HashMap<&'static str, Handler<C>>>) -> Router<C> {
        Router { route_map }
    }

    pub(crate) fn route(
        &self,
        req: Request<Body>,
        ctx: C,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<Response<Body>, hyper::Error>>
                + Send
                + Sync,
        >,
    > {
        debug!("rpc, method: {}, uri: {}", req.method(), req.uri().path());

        let route_map = self.route_map.clone();

        Box::pin(async move {
            let route_map = route_map.clone();

            let b = hyper::body::to_bytes(req.into_body()).await?;

            let json_request: JsonRequest = match serde_json::from_slice(&b) {
                Ok(r) => r,
                Err(err) => {
                    return Ok(utils::make_error_response(None, Box::new(err)));
                }
            };

            if let Some(handler) = route_map.get(json_request.method.as_str()) {
                match handler(json_request.params, ctx).await {
                    Ok(r) => return Ok(r),
                    Err(err) => {
                        return Ok(utils::make_error_response(None, err));
                    }
                }
            } else {
                return Ok(utils::make_not_found_response());
            }
        })
    }
}
