use super::Middleware;
use futures::Future;
use hyper::{Body, Request, Response};
use sak_logger::error;
use std::{pin::Pin, sync::Arc};

pub enum MiddlewareResult<C> {
    Passing(Request<Body>, Response<Body>, C),
    End(Pin<Box<dyn Future<Output = Result<Response<Body>, hyper::Error>> + Send>>),
    // End(Pin<Box<dyn Future<Output = Result<Response<Body>, hyper::Error>> + Send + Sync>>),
}

pub struct StateMachine<C> {
    pub middlewares: Arc<Vec<Middleware<C>>>,
}

impl<C> StateMachine<C> {
    pub fn run(
        &self,
        req: Request<Body>,
        res: Response<Body>,
        ctx: C,
    ) -> Pin<Box<dyn Future<Output = Result<Response<Body>, hyper::Error>> + Send>> {
        let mut rq = req;
        let mut rs = res;
        let mut ct = ctx;

        for m in self.middlewares.iter() {
            let f = &m.0;

            match f(rq, rs, ct) {
                MiddlewareResult::Passing(req, res, ctx) => {
                    rq = req;
                    rs = res;
                    ct = ctx;

                    continue;
                }
                MiddlewareResult::End(res) => return res,
            }
        }

        Box::pin(async {
            error!(
                "State machine reached the end without HandleResult \
                being terminated"
            );
            let res = Response::default();
            Ok(res)
        })
    }
}
