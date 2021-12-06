use bellman::gadgets::boolean::{AllocatedBit, Boolean};
use bellman::groth16::{Parameters, Proof};
use bellman::{groth16, Circuit, ConstraintSystem, SynthesisError};
use bls12_381::{Bls12, MillerLoopResult, Scalar};
use bytes::BytesMut;
use ff::PrimeField;
use ff::{Field, PrimeFieldBits};
use proofs::constants::get_round_constants;
use proofs::{get_merkle_tree, verify_proof};
use rand::prelude::ThreadRng;
use rand::rngs::OsRng;
use rand::thread_rng;
use rsa::pkcs8::{FromPrivateKey, ToPrivateKey};
use rsa::{PaddingScheme, PublicKey, RsaPrivateKey, RsaPublicKey};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;


pub struct Chat {
    pub cid: String,
    pub stream: TcpStream,
    pub priv_key: RsaPrivateKey,
    pub pub_key: RsaPublicKey,
    pub channel: Vec<u8>,
}

impl Chat {
    pub async fn start(&mut self) {
        // let priv_key = RsaPrivateKey::from_pkcs8_pem(&KEY)
        //     .expect("failed to generate a key");
        // let pub_key = RsaPublicKey::from(&priv_key);
        // let channel_data = b"channel_id";
        // let padding = PaddingScheme::new_pkcs1v15_encrypt();
        // let channel = pub_key
        //     .encrypt(&mut OsRng, padding, &channel_data[..])
        //     .expect("channel should be encrypted");

        println!("Shared Private key: {:?}", self.priv_key);
        println!("Channel id: {:?}", self.channel);

        let tree = get_merkle_tree(&get_round_constants());
        println!("[ledger] Transactions up to so far: {:?}\n", tree.data);

        println!("Start typing, cid: {}", self.cid);

        loop {
            let mut buffer = String::new();
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("invalid message");

            print!("You: {} [channel: {:?}]", buffer, self.channel);

            let write_buf = buffer.as_bytes();
            self.stream.write(&write_buf[..]).await;

            let tid = self.cid.parse::<usize>().unwrap();
            println!("Transaction I know: {}", tid);

            let proof = proofs::generate_proof(tid);
            let verified = verify_proof(&proof);
            println!("proof: {:?}, verified: {}", proof, verified);

            let mut read_buf = BytesMut::with_capacity(256);

            self.stream.read_buf(&mut read_buf).await;

            let msg = match std::str::from_utf8(&read_buf) {
                Ok(m) => m,
                Err(err) => {
                    println!("Wrong string, bytes: {:?}", read_buf);

                    std::process::exit(1);
                }
            };

            print!("Her: {} [channel: {:?}]", msg, self.channel);
        }
    }
}
