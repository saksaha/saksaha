use std::collections::{HashMap};
use tokio::sync::Mutex;

pub enum Traffic {
    InBound,
    OutBound,
}

pub struct OngoingCalls {
    map: Mutex<HashMap<String, Traffic>>,
}

impl OngoingCalls {
    pub fn new() -> OngoingCalls {
        let map = Mutex::new(HashMap::new());

        Calls { map }
    }

    pub async fn has_call(&self, peer_ip: &String) -> bool {
        let map = self.map.lock().await;

        return map.contains_key(peer_ip);
    }

    pub async fn insert(
        &self,
        peer_ip: String,
        traffic: Traffic,
    ) -> Option<Traffic> {
        let mut map = self.map.lock().await;

        return map.insert(peer_ip, traffic);
    }

    pub async fn remove(&self, peer_id: &String) -> Option<Traffic> {
        let mut map = self.map.lock().await;

        map.remove(peer_id)
    }
}
