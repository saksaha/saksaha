use crate::Conn;
use crate::Transport;
use crate::{handshake::*, Msg, PingMsg};
use futures::{SinkExt, StreamExt};
use sak_p2p_id::Identity;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};

async fn connect_to_endpoint(endpoint: &String) -> Conn {
    match TcpStream::connect(&endpoint).await {
        Ok(s) => {
            let c = match Conn::new(s, true) {
                Ok(c) => c,
                Err(err) => {
                    log::warn!("Error creating a connection, err: {}", err);
                    panic!()
                }
            };

            log::debug!(
                "(caller) TCP connected to destination for test, \
                        peer_addr: {:?}",
                c.socket_addr,
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
            &String::from(
                "\
                7297b903877a957748b74068d63d6d566\
                148197524099fc1df5cd9e8814c66c7",
            ),
            &String::from(
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
            &String::from(
                "\
                aa99cfd91cc6f3b541d28f3e0707f9c7b\
                cf05cf495308294786ca450b501b5f2",
            ),
            &String::from(
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
    conn: Conn,
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

    transport
}

async fn handshake_recv(
    p2p_socket: TcpListener,
    my_identity: Arc<Identity>,
) -> (Transport, String) {
    let conn_id = sak_crypto::rand();

    println!("prepare to recv msg,");

    let tcp_stream = accept(p2p_socket).await.unwrap();

    let conn = Conn::new(tcp_stream, false).unwrap();

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

    // identity_1.credential.

    let conn_2 = connect_to_endpoint(&endpoint_2).await;

    let identity_1_clone = identity_1.clone();
    let identity_2_clone = identity_2.clone();

    let rand = sak_crypto::rand() as u128;

    // send
    let t1 = tokio::spawn(async move {
        let transport_1 =
            handshake_init(conn_2, identity_1_clone, identity_2_clone).await;

        println!("preparing to send msg,");

        let mut conn_1_lock = transport_1.conn.write().await;

        let msg = PingMsg { nonce: rand };

        conn_1_lock.send(Msg::Ping(msg)).await.unwrap();
    });

    let identity_2_clone = identity_2.clone();

    //recv
    let t2 = tokio::spawn(async move {
        let (transport_2, _) =
            handshake_recv(tcp_listener_2, identity_2_clone).await;

        let mut conn_2_lock = transport_2.conn.write().await;

        let (maybe_msg, _) = conn_2_lock.next_msg().await;

        let ping = match maybe_msg {
            Some(maybe_msg) => match maybe_msg {
                Ok(msg) => match msg {
                    Msg::Ping(p) => {
                        println!("ping: {:?}, rand received!", p);

                        p
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

        let rand_recvd = ping.nonce;

        assert_eq!(rand, rand_recvd, "rand should be equal");
    });

    let _ = tokio::join!(t1, t2);
}
