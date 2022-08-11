pub(in crate::node) struct MsgHandleRoutine {}

impl MsgHandleRoutine {
    pub fn run(self) {
        loop {
            let mut conn_lock = self.peer.get_transport().conn.write().await;

            let maybe_msg = conn_lock.next_msg().await;

            match maybe_msg {
                Some(maybe_msg) => match maybe_msg {
                    Ok(msg) => {
                        let _ = msg_handle::handle_msg(
                            msg,
                            &self.machine,
                            conn_lock,
                            &self.node_task_queue,
                            &self.peer,
                        )
                        .await;
                    }
                    Err(err) => {
                        warn!("Failed to parse the msg, err: {}", err);
                    }
                },
                None => {
                    warn!("Peer has ended the connection");

                    self.peer.set_peer_status(PeerStatus::Disconnected).await;

                    return;
                }
            };
        }
    }
}
