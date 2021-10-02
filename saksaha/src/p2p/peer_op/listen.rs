use std::sync::Arc;

use crate::{
    err_res, msg_err,
    node::task_manager::{MsgKind, TaskManager},
};
use logger::log;
use tokio::{
    net::TcpListener, sync::mpsc::Sender as MpscSender,
    sync::oneshot::Sender as OneshotSender,
};

pub struct Listen {
    dial_loop_tx: Arc<MpscSender<usize>>,
    task_mng: Arc<TaskManager>,
}

impl Listen {
    pub fn new(
        dial_loop_tx: Arc<MpscSender<usize>>,
        task_mng: Arc<TaskManager>,
    ) -> Listen {
        Listen {
            dial_loop_tx,
            task_mng,
        }
    }

    pub async fn start_listening(&self, peer_op_port_tx: OneshotSender<u16>) {
        let local_addr = format!("127.0.0.1:0");

        let listener = match TcpListener::bind(local_addr).await {
            Ok(l) => {
                let local_addr = match l.local_addr() {
                    Ok(addr) => addr,
                    Err(err) => {
                        let msg = msg_err!(
                            MsgKind::SetupFailure,
                            "Error getting peer op local addr, err: {}",
                            err
                        );

                        return self.task_mng.send(msg).await;
                    }
                };

                log!(DEBUG, "Start peer op listening, addr: {}\n", local_addr);

                match peer_op_port_tx.send(local_addr.port()) {
                    Ok(_) => (),
                    Err(err) => {
                        let msg = msg_err!(
                            MsgKind::SetupFailure,
                            "Error getting peer op port, err: {}",
                            err
                        );

                        return self.task_mng.send(msg).await;
                    }
                };

                l
            }
            Err(err) => {
                let msg = msg_err!(
                    MsgKind::SetupFailure,
                    "Error binding peer op endpoint, err: {}",
                    err
                );

                return self.task_mng.send(msg).await;
            }
        };

        loop {
            let (mut stream, addr) = match listener.accept().await {
                Ok(res) => res,
                Err(err) => {
                    return;
                    // return err_res!("Error accepting a request, err: {}", err);
                }
            };

            tokio::spawn(async move {
                let mut buf = [0; 1024];

                loop {
                    // let n = match
                }
            });
        }
    }
}
