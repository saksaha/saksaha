use crate::SaksahaSDKError;
use hyper::{Body, Client, Method, Request, Uri};
use log::warn;
use sak_contract_std::{CtrCallType, CtrRequest, RequestArgs};
use sak_crypto::{
    groth16, mimc, os_rng, Bls12, Circuit, Hasher, Proof, Scalar, ScalarExt,
};
use sak_proofs::{
    get_mimc_params_1_to_2, CoinProof, CoinProofCircuit1to2, MerkleTree,
    NewCoin, OldCoin, Path, ProofError, CM_TREE_DEPTH,
};
use sak_rpc_interface::{JsonRequest, JsonResponse};
use serde::{Deserialize, Serialize};
use std::{char::from_u32_unchecked, collections::HashMap, time};
use type_extension::U8Array;

pub const A: usize = 1;
pub const TREE_DEPTH: usize = 3;

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCtrRequest {
    pub ctr_addr: String,
    pub req: CtrRequest,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryCtrResponse {
    pub result: Vec<u8>,
    // pub result: String,
}

pub fn new_empty_32_temp() -> [u8; 32] {
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0,
    ]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendPourTxRequest {
    created_at: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    author_sig: String,
    ctr_addr: Option<String>,
    #[serde(with = "serde_bytes")]
    pi: Vec<u8>,
    sn_1: [u8; 32],
    // sn_2: [u8; 32],
    cm_1: [u8; 32],
    cm_2: [u8; 32],
    merkle_rt: [u8; 32],
}

impl SendPourTxRequest {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        ctr_addr: Option<String>,
        pi: Vec<u8>,
        sn_1: [u8; 32],
        // sn_2: [u8; 32],
        cm_1: [u8; 32],
        cm_2: [u8; 32],
        merkle_rt: [u8; 32],
    ) -> SendPourTxRequest {
        SendPourTxRequest {
            created_at,
            data,
            author_sig,
            ctr_addr,
            pi,
            sn_1,
            // sn_2,
            cm_1,
            cm_2,
            merkle_rt,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SendMintTxRequest {
    created_at: String,
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    author_sig: String,
    ctr_addr: Option<String>,
    cm: [u8; 32],
    v: [u8; 32],
    k: [u8; 32],
    s: [u8; 32],
}

impl SendMintTxRequest {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        ctr_addr: Option<String>,
        cm: [u8; 32],
        v: [u8; 32],
        k: [u8; 32],
        s: [u8; 32],
    ) -> SendMintTxRequest {
        SendMintTxRequest {
            created_at,
            data,
            author_sig,
            ctr_addr,
            cm,
            v,
            k,
            s,
        }
    }
}

pub async fn send_tx_pour(
    // pi: Proof<Bls12>,
    // sn_1: U8Array,
    // sn_2: U8Array,
    // cm_1: Scalar,
    // cm_2: Scalar,
    // merkle_rt: Scalar,
    ctr_addr: String,
    // req_type: String,
    // args: RequestArgs,
    ctr_request: CtrRequest,
) -> Result<JsonResponse<String>, SaksahaSDKError> {
    let endpoint_test = "http://localhost:34418/rpc/v0";

    let client = Client::new();
    let uri: Uri = { endpoint_test.parse().expect("URI should be made") };

    let body = {
        // let req = CtrRequest {
        //     req_type: req_type.clone(),
        //     args,
        //     ctr_call_type: CtrCallType::Execute,
        // };

        // *** Need to change dummy values to real values
        let send_req = SendPourTxRequest::new(
            String::from(format!("created_at_{:?}", time::SystemTime::now())),
            serde_json::to_vec(&ctr_request)?,
            String::from("author_sig_1"),
            Some(ctr_addr),
            //
            vec![11, 11, 11],        // pi
            U8Array::new_empty_32(), // sn_1
            // U8Array::new_empty_32(), // sn_2 (will be deleted)
            U8Array::new_empty_32(), // cm_1
            U8Array::new_empty_32(), // cm_2
            U8Array::new_empty_32(), // merkle_rt
        );

        let params = serde_json::to_string(&send_req)?.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "send_pour_tx".to_string(),
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

    let resp = client.request(req).await?;

    let b = hyper::body::to_bytes(resp.into_body()).await?;

    let json_response = serde_json::from_slice::<JsonResponse<String>>(&b)?;

    Ok(json_response)
}

pub async fn send_tx_mint(
    ctr_addr: Option<String>,
    req_type: String,
    args: RequestArgs,

    cm: [u8; 32],
    v: [u8; 32],
    k: [u8; 32],
    s: [u8; 32],
) -> Result<JsonResponse<String>, SaksahaSDKError> {
    let endpoint_test = "http://localhost:34418/rpc/v0";

    let client = Client::new();
    let uri: Uri = { endpoint_test.parse().expect("URI should be made") };

    let body = {
        let req = CtrRequest {
            req_type: req_type.clone(),
            args,
            ctr_call_type: CtrCallType::Execute,
        };

        let send_req = SendMintTxRequest::new(
            String::from("created_at_1"),
            serde_json::to_vec(&req)?,
            String::from("author_sig_1"),
            ctr_addr,
            cm,
            v,
            k,
            s,
        );

        let params = serde_json::to_string(&send_req)?.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "send_mint_tx".to_string(),
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

    let resp = client.request(req).await?;

    let b = hyper::body::to_bytes(resp.into_body()).await?;

    let json_response = serde_json::from_slice::<JsonResponse<String>>(&b)?;

    Ok(json_response)
}

pub async fn query_ctr(
    ctr_addr: String,
    req_type: String,
    args: RequestArgs,
) -> Result<JsonResponse<QueryCtrResponse>, SaksahaSDKError> {
    let endpoint_test = "http://localhost:34418/rpc/v0";

    let client = Client::new();
    let uri: Uri = { endpoint_test.parse().expect("URI should be made") };

    let body = {
        let req = CtrRequest {
            req_type: req_type.clone(),
            args,
            ctr_call_type: CtrCallType::Query,
        };

        let send_req = QueryCtrRequest { ctr_addr, req };
        let params = serde_json::to_vec(&send_req)?;

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "query_ctr".to_string(),
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

    let resp = client.request(req).await?;

    let b = hyper::body::to_bytes(resp.into_body()).await?;

    let json_response =
        serde_json::from_slice::<JsonResponse<QueryCtrResponse>>(&b)?;

    // println!("json_response: {:?}", json_response);
    Ok(json_response)
}

pub async fn generate_proof_1_to_2(
    // coin_1_old: OldCoin,
    coin_1_old: OldCoin,
    coin_1_new: NewCoin,
    coin_2_new: NewCoin,
) -> Result<Proof<Bls12>, ProofError> {
    let hasher = Hasher::new();
    let constants = hasher.get_mimc_constants().to_vec();

    let de_params = sak_proofs::get_mimc_params_1_to_2(&constants);

    let c = CoinProofCircuit1to2 {
        hasher,
        coin_1_old,
        coin_1_new,
        coin_2_new,
        constants,
    };

    let proof = match groth16::create_random_proof(c, &de_params, &mut os_rng())
    {
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

pub async fn verify_proof_1_to_2(
    proof: Proof<Bls12>,
    public_inputs: &[Scalar],
    hasher: &Hasher,
) -> bool {
    let constants = hasher.get_mimc_constants();
    let de_params = get_mimc_params_1_to_2(&constants);
    let pvk = groth16::prepare_verifying_key(&de_params.vk);

    match groth16::verify_proof(&pvk, &proof, public_inputs) {
        Ok(_) => {
            println!("verify success!");
            true
        }
        Err(err) => {
            println!("verify_proof(), err: {}", err);
            false
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAuthPathRequest {
    pub cm_idx: u128,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAuthPathResponse {
    pub auth_path: Vec<([u8; 32], bool)>,
}

pub async fn get_auth_path(
    idx: u128,
) -> Result<JsonResponse<GetAuthPathResponse>, SaksahaSDKError> {
    let endpoint_test = "http://localhost:34418/rpc/v0";

    let client = Client::new();
    let uri: Uri = { endpoint_test.parse().expect("URI should be made") };

    let body = {
        let send_req = GetAuthPathRequest { cm_idx: idx };
        let params = serde_json::to_string(&send_req)?.as_bytes().to_vec();

        let json_request = JsonRequest {
            jsonrpc: "2.0".to_string(),
            method: "get_auth_path".to_string(),
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

    let resp = client.request(req).await?;

    let b = hyper::body::to_bytes(resp.into_body()).await?;

    println!("b: {:#?}", b);

    let json_response =
        serde_json::from_slice::<JsonResponse<GetAuthPathResponse>>(&b)?;
    println!("[+] idx: {:?}", idx);

    Ok(json_response)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenProofRequest {
    // coin_1_old
    pub addr_pk_1_old: [u8; 32],
    pub addr_sk_1_old: [u8; 32],
    pub rho_1_old: [u8; 32],
    pub r_1_old: [u8; 32],
    pub s_1_old: [u8; 32],
    pub v_1_old: [u8; 32],
    pub cm_1_old: [u8; 32],
    pub auth_path_1_old: [Option<([u8; 32], bool)>; CM_TREE_DEPTH as usize],

    // coin_1_new
    pub addr_pk_1_new: [u8; 32],
    pub rho_1_new: [u8; 32],
    pub r_1_new: [u8; 32],
    pub s_1_new: [u8; 32],
    pub v_1_new: [u8; 32],

    // coin_2_new
    pub addr_pk_2_new: [u8; 32],
    pub rho_2_new: [u8; 32],
    pub r_2_new: [u8; 32],
    pub s_2_new: [u8; 32],
    pub v_2_new: [u8; 32],
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GenProofResponse {
    pub pi: Vec<Option<[u8; 32]>>,
}
