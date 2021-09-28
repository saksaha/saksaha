use logger::log;

pub struct RPC {}

impl RPC {
    pub fn new() -> RPC {
        return RPC {};
    }

    pub async fn start(&self) {
        log!(DEBUG, "Start rpc...\n");
    }
}
