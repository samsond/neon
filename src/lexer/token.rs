// TokenType represents the different types of tokens in the DSL.
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    TokenEOF,
    TokenDeployApp,
    TokenNamespace,
    TokenReplicas,
    TokenImage,
    TokenEnv,
    TokenPorts,
    TokenResources,
    TokenStorage,
    TokenLBrace, // {
    TokenRBrace, // }
    TokenColon,  // :
    TokenString, // "value" or value
    TokenNumber, // 123
    TokenIdentifier,
    TokenMemory,
    TokenCPU,
    TokenVolume,
    TokenSize,
    TokenLimits,
    TokenRequests,
    TokenArgs,
    TokenService, // service
    TokenPort,
    TokenTargetPort,
    TokenTypeString,
    TokenSeparator, // ---
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_eof() {
        let token = TokenType::TokenEOF;
        assert_eq!(token, TokenType::TokenEOF);
    }

    #[test]
    fn test_token_deploy_app() {
        let token = TokenType::TokenDeployApp;
        assert_eq!(token, TokenType::TokenDeployApp);
    }

    #[test]
    fn test_token_namespace() {
        let token = TokenType::TokenNamespace;
        assert_eq!(token, TokenType::TokenNamespace);
    }

    #[test]
    fn test_token_replicas() {
        let token = TokenType::TokenReplicas;
        assert_eq!(token, TokenType::TokenReplicas);
    }

    #[test]
    fn test_token_image() {
        let token = TokenType::TokenImage;
        assert_eq!(token, TokenType::TokenImage);
    }

    #[test]
    fn test_token_env() {
        let token = TokenType::TokenEnv;
        assert_eq!(token, TokenType::TokenEnv);
    }

    #[test]
    fn test_token_ports() {
        let token = TokenType::TokenPorts;
        assert_eq!(token, TokenType::TokenPorts);
    }

    #[test]
    fn test_token_resources() {
        let token = TokenType::TokenResources;
        assert_eq!(token, TokenType::TokenResources);
    }

    #[test]
    fn test_token_storage() {
        let token = TokenType::TokenStorage;
        assert_eq!(token, TokenType::TokenStorage);
    }

    #[test]
    fn test_token_lbrace() {
        let token = TokenType::TokenLBrace;
        assert_eq!(token, TokenType::TokenLBrace);
    }

    #[test]
    fn test_token_rbrace() {
        let token = TokenType::TokenRBrace;
        assert_eq!(token, TokenType::TokenRBrace);
    }

    #[test]
    fn test_token_colon() {
        let token = TokenType::TokenColon;
        assert_eq!(token, TokenType::TokenColon);
    }

    #[test]
    fn test_token_string() {
        let token = TokenType::TokenString;
        assert_eq!(token, TokenType::TokenString);
    }

    #[test]
    fn test_token_number() {
        let token = TokenType::TokenNumber;
        assert_eq!(token, TokenType::TokenNumber);
    }

    #[test]
    fn test_token_identifier() {
        let token = TokenType::TokenIdentifier;
        assert_eq!(token, TokenType::TokenIdentifier);
    }

    #[test]
    fn test_token_memory() {
        let token = TokenType::TokenMemory;
        assert_eq!(token, TokenType::TokenMemory);
    }

    #[test]
    fn test_token_cpu() {
        let token = TokenType::TokenCPU;
        assert_eq!(token, TokenType::TokenCPU);
    }

    #[test]
    fn test_token_volume() {
        let token = TokenType::TokenVolume;
        assert_eq!(token, TokenType::TokenVolume);
    }

    #[test]
    fn test_token_size() {
        let token = TokenType::TokenSize;
        assert_eq!(token, TokenType::TokenSize);
    }

    #[test]
    fn test_token_limits() {
        let token = TokenType::TokenLimits;
        assert_eq!(token, TokenType::TokenLimits);
    }

    #[test]
    fn test_token_requests() {
        let token = TokenType::TokenRequests;
        assert_eq!(token, TokenType::TokenRequests);
    }

    #[test]
    fn test_token_args() {
        let token = TokenType::TokenArgs;
        assert_eq!(token, TokenType::TokenArgs);
    }

    #[test]
    fn test_token_service() {
        let token = TokenType::TokenService;
        assert_eq!(token, TokenType::TokenService);
    }

    #[test]
    fn test_token_port() {
        let token = TokenType::TokenPort;
        assert_eq!(token, TokenType::TokenPort);
    }

    #[test]
    fn test_token_target_port() {
        let token = TokenType::TokenTargetPort;
        assert_eq!(token, TokenType::TokenTargetPort);
    }

    #[test]
    fn test_token_type_string() {
        let token = TokenType::TokenTypeString;
        assert_eq!(token, TokenType::TokenTypeString);
    }

    #[test]
    fn test_token_separator() {
        let token = TokenType::TokenSeparator;
        assert_eq!(token, TokenType::TokenSeparator);
    }
}
