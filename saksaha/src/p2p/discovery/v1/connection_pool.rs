use std::collections::HashMap;

use tokio::sync::Mutex;

pub enum Traffic {
    InBound,
    OutBound,
}

pub struct ConnectionPool {
    map: Mutex<HashMap<String, Connection>>,
}

pub struct Connection {
    traffic: Traffic,
}

impl ConnectionPool {
    pub fn new() -> ConnectionPool {
        let map = Mutex::new(HashMap::new());

        ConnectionPool { map }
    }

    pub async fn has_call(&self, peer_ip: &String) -> bool {
        let map = self.map.lock().await;

        return map.contains_key(peer_ip);
    }

    pub async fn insert(
        &self,
        peer_ip: String,
        traffic: Traffic,
    ) -> Option<Connection> {
        let mut map = self.map.lock().await;

        let conn = Connection { traffic };

        return map.insert(peer_ip, conn);
    }

    pub async fn remove(&self, peer_id: &String) -> Option<Connection> {
        let mut map = self.map.lock().await;

        map.remove(peer_id)
    }
}
