use crate::rpc::{router::HeaderFactory, RPCError};
use hyper::{header::CONTENT_TYPE, Body, Response, StatusCode};
use sak_rpc_interface::{JsonRPCError, JsonResponse, JSON_RPC_2};
use serde::Serialize;

pub(in crate::rpc) struct RouteState {
    pub id: String,
    pub resp: Response<Body>,
}

pub(in crate::rpc) fn make_success_response<D: Serialize>(
    route_state: RouteState,
    result: D,
) -> Response<Body> {
    let header_factory = HeaderFactory::get_instance().expect(
        "Header factory should have \
            been initialized.",
    );

    let mut resp = route_state.resp;

    {
        let headers = resp.headers_mut();

        headers.insert(CONTENT_TYPE, header_factory.application_json.clone());
    }

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

pub(in crate::rpc) fn make_serialize_err_response(
    mut resp: Response<Body>,
    id: String,
    original_err: Option<RPCError>,
) -> Response<Body> {
    let header_factory = HeaderFactory::get_instance().expect(
        "Header factory should have \
            been initialized.",
    );

    resp.headers_mut()
        .insert(CONTENT_TYPE, header_factory.application_json.clone());

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

pub(in crate::rpc) fn make_not_found_response(
    route_state: RouteState,
) -> Response<Body> {
    let mut resp = route_state.resp;

    *resp.status_mut() = StatusCode::NOT_FOUND;
    *resp.body_mut() = Body::from("not found");
    resp
}

pub(in crate::rpc) fn make_error_response(
    mut resp: Response<Body>,
    id: Option<String>,
    error: RPCError,
) -> Response<Body> {
    let id = id.unwrap_or("none".to_string());

    let header_factory = HeaderFactory::get_instance().expect(
        "Header factory should have \
            been initialized.",
    );

    println!("err: {:?}", error);

    {
        let headers = resp.headers_mut();
        headers.insert(CONTENT_TYPE, header_factory.application_json.clone());
    }

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
