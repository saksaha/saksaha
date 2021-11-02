use std::collections::{HashMap};
use tokio::sync::Mutex;

pub enum Traffic {
    InBound,
    OutBound,
}

pub struct ActiveCalls {
    map: Mutex<HashMap<String, Traffic>>,
}

impl ActiveCalls {
    pub fn new() -> ActiveCalls {
        let map = Mutex::new(HashMap::new());

        ActiveCalls { map }
    }

    pub async fn contain(&self, endpoint: &String) -> bool {
        let map = self.map.lock().await;

        return map.contains_key(endpoint);
    }

    pub async fn insert(
        &self,
        endpoint: String,
        traffic: Traffic,
    ) -> Option<Traffic> {
        let mut map = self.map.lock().await;

        return map.insert(endpoint, traffic);
    }

    pub async fn remove(&self, endpoint: &String) -> Option<Traffic> {
        let mut map = self.map.lock().await;

        map.remove(endpoint)
    }
}
