use crate::rpc::router;

macro_rules! require_some_params {
    ($route_state: expr, $obj: expr, $msg: tt) => {
        match $obj {
            Some(t) => t,
            None => {
                return router::make_error_response(
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
                return router::make_error_response(
                    $route_state.resp,
                    Some($route_state.id),
                    // $route_state,
                    $msg.into(),
                );
            }
        }
    };
}

macro_rules! require_params_parsed {
    ($route_state: expr, $params: expr) => {
        match serde_json::from_slice($params) {
            Ok(r) => r,
            Err(err) => {
                return router::make_error_response(
                    $route_state.resp,
                    Some($route_state.id),
                    // $route_state,
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

pub(in crate::rpc) use require_params_parsed;
pub(in crate::rpc) use require_some_params;
