use crate::SaksahaSDKError;
use hyper::{Body, Client, Method, Request, Uri};
use sak_contract_std::{CtrCallType, Request as CtrRequest};
use sak_rpc_interface::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const A: usize = 1;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCtrRequest {
    pub ctr_addr: String,
    pub req: CtrRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCtrResponse {
    pub result: Vec<String>,
}

pub async fn query_contract(
    ctr_addr: String,
    method: String,
    arg: HashMap<String, String>,
) -> Result<JsonResponse<QueryCtrResponse>, SaksahaSDKError> {
    let endpoint_test = "http://localhost:34418/rpc/v0";

    let client = Client::new();
    let uri: Uri = { endpoint_test.parse().expect("URI should be made") };

    let body = {
        let req = CtrRequest {
            req_type: method.clone(),
            arg,
            ctr_call_type: CtrCallType::Query,
        };

        let call_ctr_req = QueryCtrRequest { ctr_addr, req };
        let params = serde_json::to_string(&call_ctr_req)?.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method,
            params: Some(params),
            id: "test_1".to_string(),
        };

        let str = serde_json::to_string(&json_request)?;

        Body::from(str)
    };

    let req = Request::builder()
        .method(Method::POST)
        .uri(uri)
        .body(body)
        .expect("request builder should be made");

    // let resp = client.request(req).await.unwrap();

    // let b = hyper::body::to_bytes(resp.into_body()).await.unwrap();
    // println!("{:?}", b);

    // let json_response =
    //     serde_json::from_slice::<JsonResponse<QueryCtrResponse>>(&b).unwrap();

    // Ok(json_response)

    {
        let json_response = JsonResponse {
            jsonrpc: "2.0".to_string(),
            error: None,
            result: Some(QueryCtrResponse {
                result: vec!["ch_1".to_string(), "ch_2".to_string()],
            }),
            id: "1312".to_string(),
        };

        return Ok(json_response);
    }

    // let resp = client.request(eq).await?;

    // let b = hyper::body::to_bytes(resp.into_body()).await?;

    // let json_response =
    //     serde_json::from_slice::<JsonResponse<QueryCtrResponse>>(&b)?;

    // Ok(json_response)
}
