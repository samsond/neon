// Contains the literals used in the deployment DSL
pub const DEPLOY_APP_PREFIX: &str = "deploy app";
pub const NAMESPACE_PREFIX: &str = "namespace:";
pub const REPLICAS_PREFIX: &str = "replicas:";
pub const IMAGE_PREFIX: &str = "image:";
pub const ENV_PREFIX: &str = "env {";
pub const ENV_TOKEN_VALUE: &str = "env";
pub const RESOURCES_PREFIX: &str = "resources {";
pub const RESOURCES_TOKEN_VALUE: &str = "resources";
pub const LIMITS_PREFIX: &str = "limits {";
pub const LIMITS_TOKEN_VALUE: &str = "limits";
pub const REQUESTS_PREFIX: &str = "requests {";
pub const REQUESTS_TOKEN_VALUE: &str = "requests";
pub const MEMORY_PREFIX: &str = "memory:";
pub const CPU_PREFIX: &str = "cpu:";
pub const STORAGE_PREFIX: &str = "storage {";
pub const STORAGE_TOKEN_VALUE: &str = "storage";
pub const VOLUME_PREFIX: &str = "volume:";
pub const SIZE_PREFIX: &str = "size:";

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
