use super::address::Address;
use futures::Future;
use log::{debug, error, info, warn};
use rand::prelude::*;
use sak_crypto::Signature;
use sak_p2p_identity::PUBLIC_KEY_LEN;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex, MutexGuard, OwnedMutexGuard,
};

const CAPACITY: usize = 32;

type Nodes = HashMap<String, Arc<TableNode>>;

pub struct Table {
    map: Mutex<Nodes>,
    keys: Mutex<Vec<String>>,
    rng: Mutex<StdRng>,
    slots_tx: Sender<Arc<TableNode>>,
    slots_rx: Mutex<Receiver<Arc<TableNode>>>,
}

impl Table {
    pub async fn init(
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Result<Table, String> {
        let (slots_tx, slots_rx) = {
            let (tx, rx) = mpsc::channel::<Arc<TableNode>>(CAPACITY);

            for _ in 0..CAPACITY {
                let empty_node = Arc::new(TableNode::new_empty());

                match tx.send(empty_node).await {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(format!(
                            "Can't send empty TableNode to the pool, err: {}",
                            err
                        ));
                    }
                }
            }

            (tx, rx)
        };

        let map = HashMap::with_capacity(CAPACITY);
        let keys = Vec::new();
        let rng = SeedableRng::from_entropy();

        let table = Table {
            map: Mutex::new(map),
            keys: Mutex::new(keys),
            rng: Mutex::new(rng),
            slots_tx,
            slots_rx: Mutex::new(slots_rx),
        };

        {
            let addrs =
                Table::convert_to_addrs(bootstrap_urls, default_bootstrap_urls);

            for addr in addrs {
                let table_node = match table.reserve().await {
                    Ok(n) => n,
                    Err(err) => {
                        return Err(format!(
                            "Couldn't initialize table, err: {}",
                            err
                        ));
                    }
                };

                table.update(table_node, |mut n| {
                    *n = TableNodeInner {
                        addr:
                    };
                }).await;
            }
        }

        Ok(table)
    }

    pub fn convert_to_addrs(
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Vec<Address> {
        let bootstrap_urls = match bootstrap_urls {
            Some(u) => u,
            None => Vec::new(),
        };

        let default_bootstrap_urls: Vec<String> = default_bootstrap_urls
            .lines()
            .map(|l| l.to_string())
            .collect();

        let urls = [bootstrap_urls, default_bootstrap_urls].concat();

        info!("*********************************************************");
        info!("* Discovery table bootstrapped");

        let mut count = 0;
        let mut addrs = vec![];
        {
            for url in urls {
                let addr = match Address::parse(url.clone()) {
                    Ok(n) => {
                        count += 1;
                        n
                    }
                    Err(err) => {
                        warn!(
                            "Discarding url failed to parse, url: {}, \
                            err: {:?}",
                            url.clone(),
                            err,
                        );

                        continue;
                    }
                };

                info!("* [{}] {}", count, addr.short_url());
                addrs.push(addr);
            }
        }

        info!("* bootstrapped node count: {}", count);
        info!("*********************************************************");

        addrs
    }

    pub async fn start(&self) -> Result<(), String> {
        for _ in 0..CAPACITY {
            let empty_node = Arc::new(TableNode::new_empty());

            match self.slots_tx.send(empty_node).await {
                Ok(_) => (),
                Err(err) => {
                    return Err(format!(
                        "Can't send empty TableNode to the pool, err: {}",
                        err
                    ));
                }
            }
        }

        Ok(())
    }

    pub async fn find(&self, endpoint: &String) -> Option<Arc<TableNode>> {
        let map = self.map.lock().await;

        if let Some(n) = map.get(endpoint) {
            return Some(n.clone());
        } else {
            return None;
        }
    }

    pub async fn find_or_reserve(
        &self,
        endpoint: &String,
    ) -> Result<Arc<TableNode>, String> {
        match self.find(endpoint).await {
            Some(n) => return Ok(n),
            None => return self.reserve().await,
        };
    }

    pub async fn find_or_try_reserve(
        &self,
        endpoint: &String,
    ) -> Result<Arc<TableNode>, String> {
        match self.find(endpoint).await {
            Some(n) => return Ok(n),
            None => return self.try_reserve().await,
        };
    }

    pub async fn update<F>(
        &self,
        table_node: Arc<TableNode>,
        updater: F,
    ) -> Result<(), String>
    where
        F: Fn(MutexGuard<Option<TableNodeInner>>),
    {
        let inner = table_node.inner.lock().await;
        let _inner = updater(inner);

        Ok(())
    }

    // pub async fn _insert(&self, addr: Address) {
    //     let mut map = self.map.lock().await;
    //     let mut indices = self.indices.lock().await;

    //     let endpoint = addr.endpoint();
    //     let node = TableNode::new(addr);

    //     map.insert(endpoint.clone(), Arc::new(Mutex::new(node)));
    //     indices.push(endpoint);
    // }

    // pub async fn register(
    //     &self,
    //     endpoint: String,
    //     table_node: Arc<TableNode>,
    // ) -> Result<(), String> {
    //     let mut map = self.map.lock().await;
    //     let mut keys = self.keys.lock().await;

    //     map.insert(endpoint.clone(), table_node);
    //     keys.push(endpoint);

    //     Ok(())
    // }

    // pub async fn next(&self) -> Option<TableNode> {
    //     let map = self.map.lock().await;
    //     let keys = self.keys.lock().await;
    //     let mut rng = self.rng.lock().await;
    //     let seed: usize = rng.gen();

    //     for i in 0..3 {
    //         let idx = (seed + i) % keys.len();
    //         let key = match keys.get(idx) {
    //             Some(k) => k,
    //             None => {
    //                 error!("Table key of idx: {}, not found", idx);
    //                 continue;
    //             }
    //         };

    //         let node = match map.get(key) {
    //             Some(n) => n.clone(),
    //             None => {
    //                 error!(
    //                     "None TableNode, something might be wrong, idx: {}",
    //                     idx,
    //                 );
    //                 return None;
    //             }
    //         };

    //         return Some(node);
    //     }

    //     None
    // }

    async fn reserve(&self) -> Result<Arc<TableNode>, String> {
        let mut slots_rx = self.slots_rx.lock().await;

        match slots_rx.recv().await {
            Some(n) => return Ok(n),
            None => return Err(format!("Can't retrieve tableNode from pool")),
        };
    }

    async fn try_reserve(&self) -> Result<Arc<TableNode>, String> {
        let mut slots_rx = self.slots_rx.lock().await;

        match slots_rx.try_recv() {
            Ok(n) => Ok(n),
            Err(err) => Err(format!(
                "Can't reserve a tableNode. Table might be busy, err: {}",
                err
            )),
        }
    }
}

pub struct TableNode {
    inner: Mutex<Option<TableNodeInner>>,
    // pub record: Option<Record>,
    // _should_be_constructed_by_table: bool,
}

pub struct TableNodeInner {
    pub addr: Address,
    pub sig: Signature,
    pub p2p_port: u16,
    pub public_key_bytes: [u8; PUBLIC_KEY_LEN],
}

impl TableNode {
    fn new_empty() -> TableNode {
        TableNode {
            inner: Mutex::new(None),
            // addr: None,
            // record: None,
            // _should_be_constructed_by_table: true,
        }
    }
}

pub struct Record {
    pub sig: Signature,
    pub p2p_port: u16,
    pub public_key_bytes: [u8; PUBLIC_KEY_LEN],
}
