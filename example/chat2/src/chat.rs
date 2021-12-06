use bytes::BytesMut;
use proofs::constants::get_round_constants;
use proofs::{verify_proof, get_merkle_tree};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use bellman::gadgets::boolean::{AllocatedBit, Boolean};
use bellman::groth16::{Parameters, Proof};
use bellman::{groth16, Circuit, ConstraintSystem, SynthesisError};
use bls12_381::{Bls12, MillerLoopResult, Scalar};
use ff::PrimeField;
use ff::{Field, PrimeFieldBits};
use rand::rngs::OsRng;
use rand::thread_rng;

pub struct Chat {
    pub cid: String,
    pub stream: TcpStream,
}

impl Chat {
    pub async fn start(&mut self) {
        println!("Start typing, cid: {}", self.cid);

        let tree = get_merkle_tree(&get_round_constants());
        println!("Transactions up to so far: {:?}", tree.data);

        loop {
            let mut buffer = String::new();
            std::io::stdin()
                .read_line(&mut buffer)
                .expect("invalid message");

            print!("You: {}", buffer);

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

            print!("Her: {}", msg);
        }
    }
}
