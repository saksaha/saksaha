use super::{discovery::Disc, peer_op::PeerOp, peer_store::PeerStore};
use crate::{
    common::SakResult,
    err_res,
    node::task_manager::{Msg, MsgKind, TaskManager},
    sync::Sync,
};
use k256::{ecdsa::SigningKey, SecretKey};
use logger::log;
use std::sync::Arc;
use tokio::{
    sync::{mpsc::Sender, Mutex},
    task::JoinHandle,
};

pub struct Host {
    rpc_port: usize,
    disc_port: usize,
    bootstrap_peers: Option<Vec<String>>,
    task_mng: Arc<TaskManager>,
    secret: String,
}

impl Host {
    pub fn new(
        rpc_port: usize,
        disc_port: usize,
        bootstrap_peers: Option<Vec<String>>,
        public_key: String,
        secret: String,
        task_mng: Arc<TaskManager>,
    ) -> SakResult<Host> {
        pub use k256::{
            ecdh::EphemeralSecret,
            ecdsa::{Signature, SigningKey, VerifyingKey},
            elliptic_curve::sec1::ToEncodedPoint,
            EncodedPoint, PublicKey, SecretKey,
        };
        use rand_core::OsRng;

        let sk = SecretKey::random(&mut OsRng);
        let sk2 = sk.to_bytes();
        let sk3 = sk2.as_slice();

        let sk4 = crate::crypto::encode_hex(sk3);
        let secret_bytes = sk4.as_bytes();

        let sk = SecretKey::from_bytes(secret_bytes).unwrap();
        // let z: SecretKey = decode_secret_key(sk4);
        // let sk_str = sk_bytes.

        println!("sk_bytes: {:?} {:?}, {:?}", sk3, sk4, sk.public_key());

        let a = crate::crypto::decode_hex(sk4.as_str()).unwrap();
        let b = SecretKey::from_bytes(a.to_owned()).unwrap();
        println!("a: {:?}, {:?}, {:?}", a, b, b.public_key());


        // let a = crate::cryp


        // SigningKey::from(sk);

        // let s = secret.as_bytes();
        // println!("s: {:?}", s);

        // let sk = SigningKey::from_bytes(s);
        // println!("sk: {:?}", sk);

        let host = Host {
            rpc_port,
            disc_port,
            bootstrap_peers,
            task_mng,
            secret,
        };

        Ok(host)
    }
}

impl Host {
    pub async fn start(&self) {
        log!(DEBUG, "Start host...\n");

        let peer_store = Arc::new(PeerStore::new(10));
        let peer_store_clone = peer_store.clone();

        let task_mng = self.task_mng.clone();

        let disc = Disc::new(
            self.disc_port,
            self.bootstrap_peers.to_owned(),
            peer_store_clone,
            task_mng,
            self.secret.to_owned(),
        );

        tokio::spawn(async move {
            disc.start().await;
        });

        let peer_store_clone = peer_store.clone();
        let peer_op = PeerOp::new(peer_store_clone);

        tokio::spawn(async move {
            peer_op.start().await;
        });
    }
}
