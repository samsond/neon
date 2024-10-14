// Contains the common literals used in the common DSL
const SEPARATOR_VALUE: &str = "---";
const RIGHT_BRACE_VALUE: &str = "}";
const LEFT_BRACE_VALUE: &str = "{";
const PORTS_PREFIX: &str = "ports {";
const PORTS_TOKEN_VALUE: &str = "ports";
const PORT_PREFIX: &str = "port:";
const TARGET_PORT_PREFIX: &str = "targetPort:";

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_separator_value() {
        assert_eq!(SEPARATOR_VALUE, "---");
    }

    #[test]
    fn test_right_brace_value() {
        assert_eq!(RIGHT_BRACE_VALUE, "}");
    }

    #[test]
    fn test_left_brace_value() {
        assert_eq!(LEFT_BRACE_VALUE, "{");
    }

    #[test]
    fn test_ports_prefix() {
        assert_eq!(PORTS_PREFIX, "ports {");
    }

    #[test]
    fn test_ports_token_value() {
        assert_eq!(PORTS_TOKEN_VALUE, "ports");
    }

    #[test]
    fn test_port_prefix() {
        assert_eq!(PORT_PREFIX, "port:");
    }

    #[test]
    fn test_target_port_prefix() {
        assert_eq!(TARGET_PORT_PREFIX, "targetPort:");
    }
}
