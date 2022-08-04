use crate::{header, RPCRouterError};
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Response, StatusCode,
};
use sak_rpc_interface::{JsonRPCError, JsonResponse, JSON_RPC_2};
use serde::Serialize;

pub struct RouteState {
    pub id: String,
    pub resp: Response<Body>,
}

pub fn make_success_response<D: Serialize>(
    route_state: RouteState,
    result: D,
) -> Response<Body> {
    let mut resp = route_state.resp;

    header::add_application_json_header(&mut resp);

    *resp.status_mut() = StatusCode::OK;

    *resp.body_mut() = {
        let response = JsonResponse {
            jsonrpc: JSON_RPC_2.into(),
            error: None,
            result: Some(result),
            id: route_state.id.to_string(),
        };

        let body_str = match serde_json::to_string(&response) {
            Ok(s) => s,
            Err(err) => {
                return make_serialize_err_response(
                    resp,
                    route_state.id,
                    Some(err.into()),
                );
            }
        };

        Body::from(body_str)
    };

    resp
}

pub fn make_serialize_err_response(
    mut resp: Response<Body>,
    id: String,
    original_err: Option<RPCRouterError>,
) -> Response<Body> {
    header::add_application_json_header(&mut resp);

    *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

    let err = if let Some(e) = original_err {
        e.to_string()
    } else {
        "".into()
    };

    *resp.body_mut() = {
        let msg = format!(
            r#"
    {{
        id: {},
        error: {{
            msg: Cannot serialize error, original err (if any): {}
        }}
    }}
    "#,
            id, err,
        );

        Body::from(msg)
    };

    resp
}

pub fn make_not_found_response(route_state: RouteState) -> Response<Body> {
    let mut resp = route_state.resp;

    *resp.status_mut() = StatusCode::NOT_FOUND;
    *resp.body_mut() = Body::from("not found");
    resp
}

pub fn make_error_response(
    mut resp: Response<Body>,
    id: Option<String>,
    error: RPCRouterError,
) -> Response<Body> {
    let id = id.unwrap_or("none".to_string());

    header::add_application_json_header(&mut resp);

    *resp.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

    *resp.body_mut() = {
        let response: JsonResponse<()> = JsonResponse {
            jsonrpc: JSON_RPC_2.into(),
            error: Some(JsonRPCError {
                msg: error.to_string(),
            }),
            result: None,
            id: id.to_string(),
        };

        let body_str = match serde_json::to_string(&response) {
            Ok(s) => s,
            Err(err) => {
                return make_serialize_err_response(
                    resp,
                    id,
                    // route_state,
                    Some(err.into()),
                );
            }
        };

        Body::from(body_str)
    };

    resp
}
