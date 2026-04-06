//! P2P DHT 路由 - 去中心化节点发现
use std::collections::HashMap;

pub struct DHTRouter {
    routing_table: HashMap<String, String>,
}

impl DHTRouter {
    pub fn new() -> Self {
        DHTRouter { routing_table: HashMap::new() }
    }

    pub fn register_node(&mut self, node_id: &str, addr: &str) {
        self.routing_table.insert(node_id.to_string(), addr.to_string());
    }

    pub fn find_node(&self, node_id: &str) -> Option<&String> {
        self.routing_table.get(node_id)
    }
}
