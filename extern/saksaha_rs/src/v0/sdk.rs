use crate::SaksahaSDKError;
use hyper::{Body, Client, Method, Request, Uri};
use rand::rngs::OsRng;
use sak_contract_std::{CtrCallType, Request as CtrRequest};
use sak_crypto::{groth16, mimc, Bls12, Circuit, Hasher, Proof, Scalar};
use sak_proofs::{
    CoinProof, CoinProofCircuit1to2, NewCoin, OldCoin, ProofError,
};
use sak_rpc_interface::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};
use std::{char::from_u32_unchecked, collections::HashMap};

pub const A: usize = 1;
pub const TREE_DEPTH: usize = 3;

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

// -------------------------------- proof

fn generate_proof_1_to_2(
    coin_1_old: OldCoin,
    coin_1_new: NewCoin,
    coin_2_new: NewCoin,
) -> Result<Proof<Bls12>, ProofError> {
    let hasher = Hasher::new();
    let constants = hasher.get_mimc_constants().to_vec();

    let de_params = sak_proofs::get_1_to_2_params(&constants);

    let c = CoinProofCircuit1to2 {
        hasher,
        coin_1_old,
        coin_1_new,
        coin_2_new,
        constants,
    };

    let proof = match groth16::create_random_proof(c, &de_params, &mut OsRng) {
        Ok(p) => p,
        Err(err) => {
            return Err(format!(
                "Failed to generate groth16 proof, err: {}",
                err
            )
            .into());
        }
    };

    Ok(proof)
}
