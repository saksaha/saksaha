use hyper::{Body, Method, Request, Response, Server, StatusCode};

pub fn create_tx(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    println!("asdfasdf");

    // tx_maker::make_tx();

    let body = Body::from("power");

    Ok(Response::new(body))
}
