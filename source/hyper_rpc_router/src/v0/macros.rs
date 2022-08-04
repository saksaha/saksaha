#[macro_export]
macro_rules! require_some_params {
    ($route_state: expr, $obj: expr, $msg: tt) => {
        match $obj {
            Some(t) => t,
            None => {
                return hyper_rpc_router::make_error_response(
                    $route_state.resp,
                    Some($route_state.id),
                    $msg.into(),
                );
            }
        }
    };
    ($route_state: expr, $obj: expr, $msg: tt,) => {
        match $obj {
            Some(t) => t,
            None => {
                return hyper_rpc_router::make_error_response(
                    $route_state.resp,
                    Some($route_state.id),
                    $msg.into(),
                );
            }
        }
    };
}

#[macro_export]
macro_rules! require_params_parsed {
    ($route_state: expr, $params: expr) => {
        match serde_json::from_slice($params) {
            Ok(r) => r,
            Err(err) => {
                return hyper_rpc_router::make_error_response(
                    $route_state.resp,
                    Some($route_state.id),
                    err.into(),
                );
            }
        }
    };
    ($route_state: expr, $params: expr,) => {
        match serde_json::from_slice($params) {
            Ok(r) => r,
            Err(err) => {
                return router::make_error_response(
                    $route_state.resp,
                    Some($route_state.id),
                    err.into(),
                );
            }
        }
    };
}
