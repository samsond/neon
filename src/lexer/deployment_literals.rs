// Contains the literals used in the deployment DSL
const DEPLOY_APP_PREFIX: &str = "deploy app";
const NAMESPACE_PREFIX: &str = "namespace:";
const REPLICAS_PREFIX: &str = "replicas:";
const IMAGE_PREFIX: &str = "image:";
const ENV_PREFIX: &str = "env {";
const ENV_TOKEN_VALUE: &str = "env";
const RESOURCES_PREFIX: &str = "resources {";
const RESOURCES_TOKEN_VALUE: &str = "resources";
const LIMITS_PREFIX: &str = "limits {";
const LIMITS_TOKEN_VALUE: &str = "limits";
const REQUESTS_PREFIX: &str = "requests {";
const REQUESTS_TOKEN_VALUE: &str = "requests";
const MEMORY_PREFIX: &str = "memory:";
const CPU_PREFIX: &str = "cpu:";
const STORAGE_PREFIX: &str = "storage {";
const STORAGE_TOKEN_VALUE: &str = "storage";
const VOLUME_PREFIX: &str = "volume:";
const SIZE_PREFIX: &str = "size:";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literals() {
        assert_eq!(DEPLOY_APP_PREFIX, "deploy app");
        assert_eq!(NAMESPACE_PREFIX, "namespace:");
        assert_eq!(REPLICAS_PREFIX, "replicas:");
        assert_eq!(IMAGE_PREFIX, "image:");
        assert_eq!(ENV_PREFIX, "env {");
        assert_eq!(ENV_TOKEN_VALUE, "env");
        assert_eq!(RESOURCES_PREFIX, "resources {");
        assert_eq!(RESOURCES_TOKEN_VALUE, "resources");
        assert_eq!(LIMITS_PREFIX, "limits {");
        assert_eq!(LIMITS_TOKEN_VALUE, "limits");
        assert_eq!(REQUESTS_PREFIX, "requests {");
        assert_eq!(REQUESTS_TOKEN_VALUE, "requests");
        assert_eq!(MEMORY_PREFIX, "memory:");
        assert_eq!(CPU_PREFIX, "cpu:");
        assert_eq!(STORAGE_PREFIX, "storage {");
        assert_eq!(STORAGE_TOKEN_VALUE, "storage");
        assert_eq!(VOLUME_PREFIX, "volume:");
        assert_eq!(SIZE_PREFIX, "size:");
    }
}