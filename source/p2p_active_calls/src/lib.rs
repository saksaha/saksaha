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

    pub async fn contains(&self, ip: &String) -> bool {
        let map = self.map.lock().await;

        return map.contains_key(ip);
    }

    pub async fn insert(
        &self,
        ip: String,
        traffic: Traffic,
    ) -> Option<Traffic> {
        let mut map = self.map.lock().await;

        return map.insert(ip, traffic);
    }

    pub async fn insert_inbound(
        &self,
        ip: String,
    ) -> Option<Traffic> {
        let mut map = self.map.lock().await;

        return map.insert(ip, Traffic::InBound);
    }

    pub async fn insert_outbound(
        &self,
        ip: String,
    ) -> Option<Traffic> {
        let mut map = self.map.lock().await;

        return map.insert(ip, Traffic::OutBound);
    }

    pub async fn remove(&self, ip: String) -> Option<Traffic> {
        let mut map = self.map.lock().await;

        map.remove(&ip)
    }
}
