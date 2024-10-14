use neon::nodes::node::Node; // Import the Node trait
use neon::nodes::service_node::ServiceNode;

fn main() {
    let service_node = ServiceNode {
        name: "my-service".to_string(),
        namespace: "default".to_string(),
        ports: [(80, 8080)].iter().cloned().collect(),
        labels: [("app".to_string(), "my-app".to_string())]
            .iter()
            .cloned()
            .collect(),
    };

    println!("Node type: {}", service_node.node_type());
}
