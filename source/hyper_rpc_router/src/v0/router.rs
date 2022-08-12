use super::{response, Handler, RouteState};
use hyper::{Body, Request, Response};
use hyper_server::MiddlewareResult;
use sak_rpc_interface::JsonRequest;
use std::{collections::HashMap, sync::Arc};

pub struct Router<C> {
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

    pub fn route(
        &self,
        req: Request<Body>,
        resp: Response<Body>,
        ctx: C,
    ) -> MiddlewareResult<C> {
        let route_map = self.route_map.clone();

        let result = Box::pin(async move {
            let route_map = route_map.clone();

            let rb = match hyper::body::to_bytes(req.into_body()).await {
                Ok(b) => b,
                Err(err) => {
                    return Ok(response::make_error_response(
                        resp,
                        None,
                        err.into(),
                    ));
                }
            };

            let json_request: JsonRequest = match serde_json::from_slice(&rb) {
                Ok(r) => r,
                Err(err) => {
                    return Ok(response::make_error_response(
                        resp,
                        None,
                        format!(
                            "Failed to parse as json_request, err: {}",
                            err
                        )
                        .into(),
                    ));
                }
            };

            println!("123123, json req: {:?}", json_request);

            let route_state = RouteState {
                id: json_request.id,
                resp,
            };

            if let Some(handler) = route_map.get(json_request.method.as_str()) {
                println!("333 found handler,");

                let resp = handler(route_state, json_request.params, ctx).await;

                println!("444 resp: {:?}", resp);

                Ok(resp)
            } else {
                return Ok(response::make_not_found_response(route_state));
            }
        });

        MiddlewareResult::End(result)
    }
}
