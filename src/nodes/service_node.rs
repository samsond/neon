use crate::nodes::node::Node;
use std::collections::HashMap;

// Define the ServiceNode struct
pub struct ServiceNode {
    pub name: String,
    pub namespace: String,
    pub ports: HashMap<i32, i32>, // Map of `port,targetPort`
    pub labels: HashMap<String, String>,
}

// Implement the Node trait for ServiceNode
impl Node for ServiceNode {
    fn node_type(&self) -> String {
        "Service".to_string()
    }
}
