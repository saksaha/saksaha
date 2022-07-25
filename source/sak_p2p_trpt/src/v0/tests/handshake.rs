use crate::net::Connection;
use crate::{handshake::*, Msg, TxHashSynMsg, UpgradedP2PCodec};
use crate::{Handshake, Transport};
use chacha20::cipher::StreamCipher;
use chacha20::{ChaCha20, ChaChaCore};
use futures::{SinkExt, StreamExt};
use log::{debug, info};
use sak_p2p_id::Identity;
use std::borrow::BorrowMut;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::{TcpListener, TcpStream};

async fn connect_to_endpoint(endpoint: &String) -> Connection {
    let conn_id = sak_crypto::rand();

    match TcpStream::connect(&endpoint).await {
        Ok(s) => {
            let c = match Connection::new(s, conn_id) {
                Ok(c) => c,
                Err(err) => {
                    log::warn!("Error creating a connection, err: {}", err);
                    panic!()
                }
            };

            log::debug!(
                "(caller) TCP connected to destination for test, \
                        peer_addr: {:?}, conn_id: {}",
                c.socket_addr,
                conn_id,
            );

            c
        }
        Err(err) => {
            log::warn!(
                "Error connecting to p2p_endpoint ({}), err: {}",
                &endpoint,
                err,
            );
            panic!()
        }
    }
}

async fn make_test_context() -> (
    Arc<Identity>,
    u16,
    String,
    //
    Arc<Identity>,
    u16,
    String,
    TcpListener,
) {
    let ip = "127.0.0.1";

    let p2p_port_1 = 35501;
    let disc_port_1 = 35518;

    let p2p_port_2 = 35502;
    let disc_port_2 = 35520;

    let identity_1 = Arc::new(
        Identity::new(
            String::from(
                "\
                7297b903877a957748b74068d63d6d566\
                148197524099fc1df5cd9e8814c66c7",
            ),
            String::from(
                "\
                045739d074b8722891c307e8e75c9607e\
                0b55a80778b42ef5f4640d4949dbf3992\
                f6083b729baef9e9545c4e95590616fd3\
                82662a09653f2a966ff524989ae8c0f",
            ),
            p2p_port_1,
            disc_port_1,
        )
        .unwrap(),
    );

    let identity_2 = Arc::new(
        Identity::new(
            String::from(
                "\
                aa99cfd91cc6f3b541d28f3e0707f9c7b\
                cf05cf495308294786ca450b501b5f2",
            ),
            String::from(
                "\
                04240874d8c323c22a571f735e835ed2\
                f0619893a3989e557b1c9b4c699ac92b\
                84d0dc478108629c0353f2876941f90d\
                4b36346bcc19c6b625422adffb53b3a6af",
            ),
            p2p_port_2,
            disc_port_2,
        )
        .unwrap(),
    );

    let endpoint_1 = format!("{}:{}", ip, p2p_port_1);
    let endpoint_2 = format!("{}:{}", ip, p2p_port_2);

    // let (tcp_listener_1, _) = sak_utils_net::bind_tcp_socket(Some(p2p_port_1))
    //     .await
    //     .unwrap();

    let (tcp_listener_2, _) = sak_utils_net::bind_tcp_socket(Some(p2p_port_2))
        .await
        .unwrap();

    (
        identity_1,
        p2p_port_1,
        endpoint_1, //
        identity_2,
        p2p_port_2,
        endpoint_2,
        tcp_listener_2,
    )
}

async fn accept(p2p_socket: TcpListener) -> Result<TcpStream, String> {
    loop {
        match p2p_socket.accept().await {
            Ok((socket, _)) => return Ok(socket),
            Err(err) => {
                return Err(err.to_string());
            }
        }
    }
}

async fn handshake_init(
    conn: Connection,
    // p2p_socket: TcpListener,
    my_identity: Arc<Identity>,
    her_identity: Arc<Identity>,
) -> Transport {
    log::debug!(
        "[init] send handshake_syn, peer node: {:?}",
        conn.socket_addr
    );

    let handshake_init_args = HandshakeInitArgs {
        identity: my_identity.clone(),
        conn,
        public_key_str: (*her_identity).credential.public_key_str.clone(),
    };

    let transport = match initiate_handshake(handshake_init_args).await {
        Ok(t) => {
            log::info!(
                "[init] peer successfuly constructs a `shared secret key` after handshaking"
            );
            t
        }
        Err(err) => {
            log::warn!(
                "Error processing InitiateHandshake, discarding, \
                        err: {}",
                err,
            );

            panic!()
        }
    };

    // let mut conn_lock = transport.conn.write().await;
    // conn.write()
    transport
}

async fn handshake_recv(
    p2p_socket: TcpListener,
    my_identity: Arc<Identity>,
) -> (Transport, String) {
    let conn_id = sak_crypto::rand();

    println!("prepare to recv msg,");
    let tcp_stream = accept(p2p_socket).await.unwrap();

    let conn = Connection::new(tcp_stream, conn_id).unwrap();

    log::debug!(
        "[recv] receive handshake_syn, peer node: {:?}, conn_id: {}",
        conn.socket_addr,
        conn_id,
    );

    let handshake_recv_args = HandshakeRecvArgs {
        identity: my_identity.to_owned(),
    };

    log::debug!(
        "[recv] send handshake_ack, peer node: {:?}",
        conn.socket_addr
    );

    let (transport, her_public_key_str) =
        match receive_handshake(handshake_recv_args, conn).await {
            Ok(t) => {
                log::info!(
                    "[recv] peer successfuly constructs a `shared \
                secret key` after handshaking"
                );

                t
            }
            Err(err) => {
                log::warn!(
                    "Error processing InitiateHandshake, discarding, \
                            err: {}",
                    err,
                );

                panic!();
            }
        };

    (transport, her_public_key_str)
}

#[tokio::test(flavor = "multi_thread")]
async fn test_handshake_works() {
    sak_test_utils::init_test_log();

    let (
        identity_1,
        _p2p_port_1,
        _endpoint_1,
        //
        identity_2,
        p2p_port_2,
        endpoint_2,
        tcp_listener_2,
    ) = make_test_context().await;

    let conn_2 = connect_to_endpoint(&endpoint_2).await;

    let identity_1_clone = identity_1.clone();
    let identity_2_clone = identity_2.clone();

    // send
    tokio::spawn(async move {
        let transport_1 =
            handshake_init(conn_2, identity_1_clone, identity_2_clone).await;

        println!("[111] sleep... before send msg");
        tokio::time::sleep(Duration::from_secs(2)).await;

        println!("preparing to send msg,");

        let mut conn_1_lock = transport_1.conn.write().await;

        let msg = TxHashSynMsg {
            tx_hashes: vec!["123".to_string()],
        };

        conn_1_lock.socket.send(Msg::TxHashSyn(msg)).await.unwrap();
    });

    let identity_2_clone = identity_2.clone();

    //recv
    tokio::spawn(async move {
        let (transport_2, _) =
            handshake_recv(tcp_listener_2, identity_2_clone).await;

        println!("[222] wait... before recv msg");
        let mut conn_2_lock = transport_2.conn.write().await;

        let maybe_msg = conn_2_lock.socket.next().await;
        println!("1414 maybe_msg after next: {:?}", maybe_msg);

        let result = match maybe_msg {
            Some(maybe_msg) => match maybe_msg {
                Ok(msg) => match msg {
                    Msg::TxHashSyn(msg) => {
                        println!("tx hash syn: {:?}", msg);
                        msg
                    }
                    _ => {
                        panic!();
                    }
                },
                Err(err) => {
                    println!("Err: {}", err);
                    panic!();
                }
            },
            None => {
                println!("No msg..");
                panic!();
            }
        };

        println!("expected value: {:?}", "123".to_string());
        println!("received value: {:?}", result.tx_hashes[0]);

        assert_eq!(result.tx_hashes[0], "123".to_string());
        println!("test pass!");

        // let mut count = 0;
        // loop {
        //     println!("loop start");
        //     let maybe_msg = conn_2_lock.socket.next().await;

        //     if count > 10 {
        //         return;
        //     }

        //     println!("5551313 maybe_msg after next: {:?}", maybe_msg);

        //     match maybe_msg {
        //         Some(maybe_msg) => match maybe_msg {
        //             Ok(msg) => match msg {
        //                 // Msg::Hello(hello) => {
        //                 //     println!("hello: {:?}", hello);
        //                 // }
        // Msg::TxHashSyn(msg) => {
        //     println!("tx hash syn: {:?}", msg);
        // }
        //                 _ => {
        //                     println!("invalid msg");
        //                 }
        //             },
        //             Err(err) => {
        //                 println!("alwekfj33, err: {}", err);
        //             }
        //         },
        //         None => {
        //             println!("alwekfj44");
        //             count += 1;
        //         }
        //     }
        // }
    });

    // let (trpt_1, trpt_2) = tokio::join!(transport_1, transport_2);

    // println!("234234 handshake done!!!");

    // tokio::spawn(async move {
    //     tokio::time::sleep(Duration::from_secs(4)).await;

    //     let mut conn_2_lock = trpt_2.conn.write().await;

    //     let mut count = 0;

    //     loop {
    //         if count > 10 {
    //             break;
    //         }

    //         println!("555 maybe_msg before next");

    //         let maybe_msg = conn_2_lock.socket.next().await;

    //         println!("555 maybe_msg after next: {:?}", maybe_msg);

    //         // match maybe_msg {
    //         //     Some(maybe_msg) => match maybe_msg {
    //         //         Ok(msg) => match msg {
    //         //             Msg::Hello(hello) => {
    //         //                 println!("hello: {:?}", hello);
    //         //             }
    //         //             Msg::TxHashSyn(msg) => {
    //         //                 println!("tx hash syn: {:?}", msg);
    //         //             }
    //         //             _ => {
    //         //                 panic!("invalid msg");
    //         //             }
    //         //         },
    //         //         Err(err) => {
    //         //             println!("alwekfj33, err: {}", err);
    //         //         }
    //         //     },
    //         //     None => {
    //         //         println!("alwekfj44");
    //         //         count += 1;
    //         //     }
    //         // }
    //     }

    //     tokio::time::sleep(Duration::from_secs(100)).await;
    // });

    // tokio::spawn(async move {
    //     let mut conn_1_lock = trpt_1.conn.write().await;

    //     println!("666 before send");

    //     let a = TxHashSynMsg {
    //         tx_hashes: vec!["123".to_string()],
    //     };

    //     // let b = HelloMsg { nonce: 1 };

    //     conn_1_lock.socket.send(Msg::TxHashSyn(a)).await.unwrap();

    //     // println!("666 after send");

    //     tokio::time::sleep(Duration::from_secs(100)).await;
    // });

    tokio::time::sleep(Duration::from_secs(100)).await;
}
