use crate::chat::Chat;
use crate::data::{client_0, client_1};
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
use tokio::net::{TcpListener, TcpSocket, TcpStream};
use tokio::sync::Mutex;
use tokio::{self, signal};

const KEY: &str = "-----BEGIN PRIVATE KEY-----\nMIIEvgIBADANBgkqhkiG9w0BAQEFAASCBKgwggSkAgEAAoIBAQDHhNucfrRmy9vd\nctDR+Lhf5GMljv5HmUFNQ7rfB8wqH+Zqb/PMB89zrrtWOCnwsqbbQKW1fJoe41Ad\nDfr+F5M/CWwTZQeKsRJkEzaILd4DZ9dZ2t+Y+6+LaXQqpe4+BvGhFufwiZk5O/Pn\nhQRNVW98ttVkWYZ3+JG/u/ncDovufJKVRuFXJkCCEp4pd8SNRLUz6ALapm5x1hyZ\nxKZO8Yh4/Vrm85v22gSFGDz3vtElctHTPCoD8UTe2JtAE6doEfp2I5BhNCDFv6Z+\nqYXdSYTTNoLNzV93JqupaXo3eVd8pha4SyDi2vZ4CTu2fCvDd5bteOHHCVVlEVs1\nsyIubV6tAgMBAAECggEBAJKetsiFcEtOqm9Nbdmv3sZRTu8N6x1wqUV/bXdzAn0+\nXhR/2vhoEPKhSHgu49tHXkknc98wBw6F25zOZlGNv56GbQ6/otAwGiTC3cMrecik\n6ePVbZW9954Ky9x/fdnFPpI0BBSG+bvOrAnKfYPbMgfDUIPjKmdFms389LvbUMfb\nJE4pRZn7oOjAzp5dX0xojvS1JARsbqxYtWIwGuOFNno1k+q4fm4VijfrR4cZ+OA1\nb7xPVW6qyaEEQ/8ZPlSTvaKP4jChUcKHDOP0qRWdUAsWXmoTwebgl6OVsUAJnaRx\nTgcklmYDhOOcFYYIjwLbLPJvOlsBtFG+3cIEC9XM3j0CgYEAzligZTKV36++eIl2\nAp1l9rrRARD43l74VJsxB+4NyHd3+xyVE4s8YUiHHmtmCDsQS5pEvmcsV2jSYnD4\nFUsVE70jr5kxUdkXv8+g/OYmYbrgmAaH+V7dIRtAPd1X50w+clviuxNkSxahfUU5\nDah6OJbAVj6Zcsic+ETHKCd4OpcCgYEA94eo33N3Uk2MhW+ixXOjCy38kbJfVt01\nqJinkT2UBdVewEVIZfkLqWJaJjojz4WTHRao7CAVEV+lhwppWE2B55kdR4hW0rW7\nVRYy89kbnLYwHaqq3dgwtw+jwk3hDESWjd5icl+A5v5BWslnR1taqtNbntqDcG/i\nFc72Sh88LVsCgYApUxyER9Y2HxcxQq5MpcMW36Ed6o08054+K3ptUSKXhD2WH8pH\nSLpF7WWKFQ4xFcsUqQedbkI2GzsPFfrJIOlOs7Fi6HY1IxobgvSF0X3cUrXKHHy8\n11/H0pJSEXMaUm2rL3W/64lJdHXRBb0AXD5l+OL6Ir1OfJjbK8MK6qSsbwKBgEc8\nrpgoVeXE1bUtTK4AuqKkCHUx3YhVP3UXAdql2yzWnF8/UTevtfB4krM/tkCqE42i\nkNjr3sRSP4QetfSo0cMCaM/GaB5aGODC88UD7ZNrR2uyS1xvNI3qnVl1KlhqgN2x\na6m43JMeB1cGcYeyveQ90bdk8cDxIiLenjj0xk/TAoGBAMYM+CuegiEIQCDOIN5l\nQ+ijuL5Levwb2D8ZA9U0jDiUsoNFwyOox2DWsLI7DTO3O2pvm/ODlP+YsaeVYYC+\nyQKlE2+jlLMiMmH8yTk+nGs+GQaAvULE8fOFhEuLaVTV9IsH/39n25x0wgS1Zacu\nQG9GO7oa7XbW4rRgH+VBptow\n-----END PRIVATE KEY-----\n";

pub struct ChatApp {
    cid: String,
}

impl ChatApp {
    pub fn new(cid: String) -> ChatApp {
        ChatApp { cid }
    }

    pub fn run(&self) {
        println!("Chat app run, cid: {}", self.cid);

        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build();

        match runtime {
            Ok(r) => r.block_on(async {
                start_routine(self.cid.clone()).await;

                tokio::select!(
                    c = signal::ctrl_c() => {
                        match c {
                            Ok(_) => {
                                println!("ctrl+k is pressed.");

                std::process::exit(1);
                            },
                            Err(err) => {
                                println!(
                                    "Unexpected error while waiting for \
                                        ctrl+p, err: {}",
                                    err
                                );

                                std::process::exit(1);

                            }
                        }
                    },
                );
            }),
            Err(err) => {
                std::process::exit(1);
            }
        }
    }
}

async fn start_routine(cid: String) {

    if cid.clone() == "0" {
        let priv_key =
            RsaPrivateKey::from_pkcs8_pem(&KEY).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        let channel_data = b"channel_id";
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        let channel = pub_key
            .encrypt(&mut OsRng, padding, &channel_data[..])
            .expect("channel should be encrypted");

        let my_ip = client_0::IP;
        let my_port = client_0::PORT;

        let local_addr = format!("{}:{}", my_ip, my_port);
        let client_id = cid.clone();

        tokio::spawn(async move {
            let cid = client_id.clone();

            let tcp_listener = match TcpListener::bind(local_addr).await {
                Ok(listener) => match listener.local_addr() {
                    Ok(local_addr) => {
                        println!(
                            "Listener bound the address, addr: {}",
                            local_addr
                        );
                        listener
                    }
                    Err(err) => {
                        println!(
                            "Can't get local address of tcp listener, err: {}",
                            err
                        );
                        std::process::exit(1);
                    }
                },
                Err(err) => {
                    println!("Can't bind tcp listener, err: {}", err,);
                    std::process::exit(1);
                }
            };

            let stream = match tcp_listener.accept().await {
                Ok((stream, addr)) => {
                    println!("Accepted new conneciton, addr: {:?}", addr);

                    stream
                }
                Err(err) => {
                    println!(
                        "Error accepting connection request, err: {}",
                        err,
                    );
                    std::process::exit(1);
                }
            };

            let mut chat = Chat {
                cid: cid.clone(),
                stream,
                priv_key,
                pub_key,
                channel,
            };
            chat.start().await;
        });
    }

    if cid == "1" {
        let priv_key =
            RsaPrivateKey::from_pkcs8_pem(&KEY).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        let channel_data = b"channel_id";
        let padding = PaddingScheme::new_pkcs1v15_encrypt();
        let channel = pub_key
            .encrypt(&mut OsRng, padding, &channel_data[..])
            .expect("channel should be encrypted");

        let dst_id = client_0::IP;
        let dst_port = client_0::PORT;

        let endpoint = format!("{}:{}", dst_id, dst_port);

        println!("Connecting endpoint: {}...", endpoint);

        let stream = match TcpStream::connect(&endpoint).await {
            Ok(s) => {
                println!("Connected to addr, {:?}", s.peer_addr());
                s
            }
            Err(err) => {
                println!(
                    "Cannot connect to client, cid: {}, err: {}",
                    cid.clone(),
                    err
                );

                std::process::exit(1);
            }
        };

        let mut chat = Chat {
            cid: cid.clone(),
            stream,
            priv_key,
            pub_key,
            channel,
        };
        chat.start().await;
    }
}
