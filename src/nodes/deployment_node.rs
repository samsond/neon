use std::collections::HashMap;
use crate::nodes::node::Node;

// Define the ResourceSpec struct
pub struct ResourceSpec {
    pub memory: String,
    pub cpu: String,
}

// Define the ResourceRequirementsNode struct
pub struct ResourceRequirementsNode {
    pub limits: ResourceSpec,
    pub requests: ResourceSpec,
}

// Define the StorageConfigNode struct
pub struct StorageConfigNode {
    pub volume: String,
    pub size: String,
}

// Define the DeploymentNode struct
pub struct DeploymentNode {
    pub name: String,
    pub namespace: String,
    pub replicas: i32,
    pub image: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
    pub ports: HashMap<String, i32>,
    pub resources: Option<ResourceRequirementsNode>,
    pub storage: Option<StorageConfigNode>,
}

// Implement the Node trait for DeploymentNode
impl Node for DeploymentNode {
    fn node_type(&self) -> String {
        "Deployment".to_string()
    }
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_node_creation() {
        let resource_requirements = ResourceRequirementsNode {
            limits: ResourceSpec {
                memory: "512Mi".to_string(),
                cpu: "1".to_string(),
            },
            requests: ResourceSpec {
                memory: "256Mi".to_string(),
                cpu: "0.5".to_string(),
            },
        };

        let storage_config = StorageConfigNode {
            volume: "my-volume".to_string(),
            size: "1Gi".to_string(),
        };

        let deployment_node = DeploymentNode {
            name: "my-deployment".to_string(),
            namespace: "default".to_string(),
            replicas: 3,
            image: "my-image".to_string(),
            args: vec!["arg1".to_string(), "arg2".to_string()],
            env: [("ENV_VAR".to_string(), "value".to_string())].iter().cloned().collect(),
            ports: [("http".to_string(), 80)].iter().cloned().collect(),
            resources: Some(resource_requirements),
            storage: Some(storage_config),
        };

        assert_eq!(deployment_node.name, "my-deployment");
        assert_eq!(deployment_node.namespace, "default");
        assert_eq!(deployment_node.replicas, 3);
        assert_eq!(deployment_node.image, "my-image");
        assert_eq!(deployment_node.args, vec!["arg1".to_string(), "arg2".to_string()]);
        assert_eq!(deployment_node.env.get("ENV_VAR").unwrap(), "value");
        assert_eq!(deployment_node.ports.get("http").unwrap(), &80);
        assert!(deployment_node.resources.is_some());
        assert!(deployment_node.storage.is_some());
    }

    #[test]
    fn test_deployment_node_type() {
        let deployment_node = DeploymentNode {
            name: "my-deployment".to_string(),
            namespace: "default".to_string(),
            replicas: 3,
            image: "my-image".to_string(),
            args: vec!["arg1".to_string(), "arg2".to_string()],
            env: HashMap::new(),
            ports: HashMap::new(),
            resources: None,
            storage: None,
        };

        assert_eq!(deployment_node.node_type(), "Deployment");
    }
}

