use super::{response, Handler};
use crate::rpc::middlewares::HandleResult;
use futures::Future;
use hyper::{Body, Method, Request, Response};
use log::debug;
use sak_rpc_interface::JsonRequest;
use std::{collections::HashMap, pin::Pin, sync::Arc};

pub(crate) struct Router<C> {
    route_map: Arc<HashMap<&'static str, Handler<C>>>,
}

impl<C> Router<C>
where
    C: Send + Sync + 'static,
{
    pub fn new(route_map: HashMap<&'static str, Handler<C>>) -> Router<C> {
        let route_map = Arc::new(route_map);

        Router { route_map }
    }

    pub(in crate::rpc) fn route(
        &self,
        req: Request<Body>,
        res: Response<Body>,
        ctx: C,
    ) -> HandleResult<C> {
        let route_map = self.route_map.clone();

        let result = Box::pin(async move {
            let route_map = route_map.clone();

            let b = hyper::body::to_bytes(req.into_body()).await?;

            let json_request: JsonRequest = match serde_json::from_slice(&b) {
                Ok(r) => r,
                Err(err) => {
                    return Ok(response::make_error_response(
                        res,
                        None,
                        Box::new(err),
                    ));
                }
            };

            if let Some(handler) = route_map.get(json_request.method.as_str()) {
                match handler(res, json_request.id, json_request.params, ctx)
                    .await
                {
                    Ok(r) => return Ok(r),
                    Err(err) => {
                        return Ok(response::make_error_response(
                            res, None, err,
                        ));
                    }
                }
            } else {
                return Ok(response::make_not_found_response());
            }
        });

        HandleResult::End(result)
    }
}
