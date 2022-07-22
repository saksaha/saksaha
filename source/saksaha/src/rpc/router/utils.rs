use super::{HandleError, JsonResponse, JSON_RPC_VERSION};
use crate::rpc::{router::HeaderFactory, RPCError};
use hyper::{
    header::{HeaderValue, CONTENT_TYPE},
    Body, Response, StatusCode,
};
use serde::{Deserialize, Serialize};

pub(in crate::rpc) fn make_serialize_err_response(
    id: String,
    original_err: Option<RPCError>,
) -> Response<Body> {
    let header_factory = HeaderFactory::get_instance().expect(
        "Header factory should have \
            been initialized.",
    );

    let mut res = Response::default();

    res.headers_mut()
        .insert(CONTENT_TYPE, header_factory.application_json.clone());

    *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

    let err = if let Some(e) = original_err {
        e.to_string()
    } else {
        "".into()
    };

    *res.body_mut() = {
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

    res
}

pub(in crate::rpc) fn make_not_found_response() -> Response<Body> {
    let mut res: Response<Body> = Response::default();
    *res.status_mut() = StatusCode::NOT_FOUND;
    *res.body_mut() = Body::from("not found");
    res
}

pub(in crate::rpc) fn make_error_response(error: RPCError) -> Response<Body> {
    let id = "none".to_string();

    let mut res: Response<Body> = Response::default();

    *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;

    *res.body_mut() = {
        let response: JsonResponse<()> = JsonResponse {
            jsonrpc: JSON_RPC_VERSION.into(),
            error: Some(HandleError {
                msg: error.to_string(),
            }),
            result: None,
            id: id.clone(),
        };

        let body_str = match serde_json::to_string(&response) {
            Ok(s) => s,
            Err(err) => {
                return make_serialize_err_response(id, Some(err.into()))
            }
        };

        Body::from(body_str)
    };

    res
}
