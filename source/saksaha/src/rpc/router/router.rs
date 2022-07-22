use super::{utils, Handler};
use futures::Future;
use hyper::{Body, Request, Response};
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
        println!("method: {}, req: {}", req.method(), req.uri().path());

        let route_map = self.route_map.clone();

        Box::pin(async move {
            let route_map = route_map.clone();

            if let Some(handler) = route_map.get(req.uri().path()) {
                match handler(req, ctx).await {
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
