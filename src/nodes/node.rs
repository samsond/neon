// Define the Node trait
pub trait Node {
    fn node_type(&self) -> String;
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    // Example struct implementing the Node trait for testing purposes
    struct TestNode;

    impl Node for TestNode {
        fn node_type(&self) -> String {
            "Example".to_string()
        }
    }

    #[test]
    fn test_node_type() {
        let node = TestNode;
        assert_eq!(node.node_type(), "Example");
    }
}
