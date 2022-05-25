#[cfg(test)]
mod test {
    use crate::{
        blockchain::{Blockchain, BlockchainArgs},
        machine::Machine,
        rpc::RPCArgs,
    };
    use hyper::{Body, Client, Method, Request, Uri};
    use std::{net::SocketAddr, sync::Arc};
    use tokio::net::TcpListener;

    use crate::rpc::RPC;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    async fn make_rpc() -> (RPC, TcpListener, SocketAddr) {
        let (rpc_socket, rpc_socket_addr) =
            utils_net::bind_tcp_socket(Some(12345))
                .await
                .expect("rpc socket should be initialized");

        let blockchain = {
            let blockchain_args = BlockchainArgs {
                app_prefix: "default".to_string(),
            };

            Blockchain::init(blockchain_args).await.unwrap()
        };

        let machine = {
            let m = Machine { blockchain };

            Arc::new(m)
        };

        let rpc = {
            let rpc_args = RPCArgs {
                machine: machine.clone(),
            };

            RPC::init(rpc_args).expect("RPC should be initialized")
        };

        (rpc, rpc_socket, rpc_socket_addr)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_rpc_client_and_send_wrong_transaction() {
        init();

        let (rpc, rpc_socket, rpc_socket_addr) = make_rpc().await;

        let _rpc_server =
            tokio::spawn(
                async move { rpc.run(rpc_socket, rpc_socket_addr).await },
            );

        let client = Client::new();

        let uri: Uri = {
            let u = format!(
                "http://localhost:{}/apis/v1/send_transaction",
                rpc_socket_addr.port()
            );

            u.parse().expect("URI should be made")
        };

        let req = Request::builder()
            .method(Method::POST)
            .uri(uri)
            .body(Body::from("123"))
            .expect("request builder should be made");

        let _result = match client.request(req).await {
            Ok(mut res) => {
                let body = hyper::body::to_bytes(&mut res)
                    .await
                    .expect("body should be parsed");

                let a = std::str::from_utf8(&body)
                    .expect("should be converted to string");

                println!("power: {:?}", res);
            }
            Err(err) => {
                panic!("error: {}", err);
            }
        };
    }
}
