use crate::lexer::common_literals::*;
use crate::lexer::deployment_literals::*;
use crate::lexer::token::TokenType;
use std::str::Lines;

// LexerInterface defines the contract for any lexer implementation
pub trait LexerInterface {
    fn next_token(&mut self) -> Token;
}

// Token represents a token with its type, value, and line number.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
    pub line_number: usize,
}

// Lexer struct represents a lexer for the DSL.
#[derive(Debug)]
pub struct Lexer<'a> {
    lines: Lines<'a>,
    tokens: Vec<Token>, // Store tokens for debugging
    pos: usize,         // Position for tracking the current token in tokens slice
    line_number: usize, // Current line number
}

impl<'a> Lexer<'a> {
    // NewLexer initializes a new Lexer and pre-tokenizes the input for debugging purposes.
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            lines: input.lines(),
            tokens: Vec::new(),
            pos: 0,
            line_number: 1,
        };
        lexer.tokenize(); // Pre-tokenize to store tokens without consuming.
        lexer
    }

    // tokenize reads all tokens without affecting the scanning process.
    fn tokenize(&mut self) {
        while let Some(line) = self.lines.next() {
            let text = line.trim();
            let token = self.scan_token(text);
            self.tokens.push(token);
            self.line_number += 1;
        }
        self.tokens.push(Token {
            token_type: TokenType::TokenEOF,
            value: String::new(),
            line_number: self.line_number,
        }); // Ensure EOF is included.
    }

    // scan_token processes a single line of text into a token.
    fn scan_token(&self, text: &str) -> Token {
        match text {
            _ if text == SEPARATOR_VALUE => Token {
                token_type: TokenType::TokenSeparator,
                value: SEPARATOR_VALUE.to_string(),
                line_number: self.line_number,
            },
            _ if text.starts_with(DEPLOY_APP_PREFIX) => {
                let value = text
                    .trim_start_matches(DEPLOY_APP_PREFIX)
                    .trim_end_matches(LEFT_BRACE_VALUE)
                    .trim()
                    .to_string();
                Token {
                    token_type: TokenType::TokenDeployApp,
                    value,
                    line_number: self.line_number,
                }
            }
            _ if text.starts_with(NAMESPACE_PREFIX) => Token {
                token_type: TokenType::TokenNamespace,
                value: parse_string_value(text),
                line_number: self.line_number,
            },
            _ if text.starts_with(REPLICAS_PREFIX) => Token {
                token_type: TokenType::TokenReplicas,
                value: parse_string_value(text),
                line_number: self.line_number,
            },
            _ if text.starts_with(IMAGE_PREFIX) => Token {
                token_type: TokenType::TokenImage,
                value: parse_string_value(text),
                line_number: self.line_number,
            },
            _ if text.starts_with(ENV_PREFIX) => Token {
                token_type: TokenType::TokenEnv,
                value: ENV_TOKEN_VALUE.to_string(),
                line_number: self.line_number,
            },
            _ if text.starts_with(PORTS_PREFIX) => Token {
                token_type: TokenType::TokenPorts,
                value: PORTS_TOKEN_VALUE.to_string(),
                line_number: self.line_number,
            },
            _ if text.starts_with(RESOURCES_PREFIX) => Token {
                token_type: TokenType::TokenResources,
                value: RESOURCES_TOKEN_VALUE.to_string(),
                line_number: self.line_number,
            },
            _ if text.starts_with(LIMITS_PREFIX) => Token {
                token_type: TokenType::TokenLimits,
                value: LIMITS_TOKEN_VALUE.to_string(),
                line_number: self.line_number,
            },
            _ if text.starts_with(REQUESTS_PREFIX) => Token {
                token_type: TokenType::TokenRequests,
                value: REQUESTS_TOKEN_VALUE.to_string(),
                line_number: self.line_number,
            },
            _ if text.starts_with(MEMORY_PREFIX) => Token {
                token_type: TokenType::TokenMemory,
                value: parse_string_value(text),
                line_number: self.line_number,
            },
            _ if text.starts_with(CPU_PREFIX) => Token {
                token_type: TokenType::TokenCPU,
                value: parse_string_value(text),
                line_number: self.line_number,
            },
            _ if text.starts_with(STORAGE_PREFIX) => Token {
                token_type: TokenType::TokenStorage,
                value: STORAGE_TOKEN_VALUE.to_string(),
                line_number: self.line_number,
            },
            _ if text.starts_with(VOLUME_PREFIX) => Token {
                token_type: TokenType::TokenVolume,
                value: parse_string_value(text),
                line_number: self.line_number,
            },
            _ if text.starts_with(SIZE_PREFIX) => Token {
                token_type: TokenType::TokenSize,
                value: parse_string_value(text),
                line_number: self.line_number,
            },
            _ if text == RIGHT_BRACE_VALUE => Token {
                token_type: TokenType::TokenRBrace,
                value: RIGHT_BRACE_VALUE.to_string(),
                line_number: self.line_number,
            },
            _ if text.starts_with(PORT_PREFIX) => Token {
                token_type: TokenType::TokenPort,
                value: parse_string_value(text),
                line_number: self.line_number,
            },
            _ if text.starts_with(TARGET_PORT_PREFIX) => Token {
                token_type: TokenType::TokenTargetPort,
                value: parse_string_value(text),
                line_number: self.line_number,
            },
            _ => Token {
                token_type: TokenType::TokenIdentifier,
                value: text.to_string(),
                line_number: self.line_number,
            },
        }
    }
}

// NextToken returns the next token in sequence.
impl<'a> LexerInterface for Lexer<'a> {
    fn next_token(&mut self) -> Token {
        if self.pos < self.tokens.len() {
            let token = self.tokens[self.pos].clone();
            self.pos += 1;
            token
        } else {
            Token {
                token_type: TokenType::TokenEOF,
                value: String::new(),
                line_number: self.line_number,
            }
        }
    }
}

// parse_string_value extracts and cleans up the string value from a line.
fn parse_string_value(line: &str) -> String {
    let parts: Vec<&str> = line.splitn(2, ':').collect();
    if parts.len() < 2 {
        return String::new();
    }
    let value = parts[1]
        .trim()
        .trim_matches(&['"', ';', '{', '}'][..])
        .to_string();
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_token() {
        let lexer = Lexer {
            lines: "".lines(),
            tokens: Vec::new(),
            pos: 0,
            line_number: 1,
        };

        let test_cases = vec![
            (
                "Separator",
                "---",
                Token {
                    token_type: TokenType::TokenSeparator,
                    value: SEPARATOR_VALUE.to_string(),
                    line_number: 1,
                },
            ),
            (
                "DeployApp",
                "deploy app myapp {",
                Token {
                    token_type: TokenType::TokenDeployApp,
                    value: "myapp".to_string(),
                    line_number: 1,
                },
            ),
            (
                "Namespace",
                "namespace: mynamespace",
                Token {
                    token_type: TokenType::TokenNamespace,
                    value: "mynamespace".to_string(),
                    line_number: 1,
                },
            ),
            (
                "Replicas",
                "replicas: 3",
                Token {
                    token_type: TokenType::TokenReplicas,
                    value: "3".to_string(),
                    line_number: 1,
                },
            ),
            (
                "Image",
                "image: myimage",
                Token {
                    token_type: TokenType::TokenImage,
                    value: "myimage".to_string(),
                    line_number: 1,
                },
            ),
            (
                "Env",
                "env {",
                Token {
                    token_type: TokenType::TokenEnv,
                    value: ENV_TOKEN_VALUE.to_string(),
                    line_number: 1,
                },
            ),
            (
                "Ports",
                "ports {",
                Token {
                    token_type: TokenType::TokenPorts,
                    value: PORTS_TOKEN_VALUE.to_string(),
                    line_number: 1,
                },
            ),
            (
                "Resources",
                "resources {",
                Token {
                    token_type: TokenType::TokenResources,
                    value: RESOURCES_TOKEN_VALUE.to_string(),
                    line_number: 1,
                },
            ),
            (
                "Limits",
                "limits {",
                Token {
                    token_type: TokenType::TokenLimits,
                    value: LIMITS_TOKEN_VALUE.to_string(),
                    line_number: 1,
                },
            ),
            (
                "Requests",
                "requests {",
                Token {
                    token_type: TokenType::TokenRequests,
                    value: REQUESTS_TOKEN_VALUE.to_string(),
                    line_number: 1,
                },
            ),
            (
                "Memory",
                "memory: 512Mi",
                Token {
                    token_type: TokenType::TokenMemory,
                    value: "512Mi".to_string(),
                    line_number: 1,
                },
            ),
            (
                "CPU",
                "cpu: 1",
                Token {
                    token_type: TokenType::TokenCPU,
                    value: "1".to_string(),
                    line_number: 1,
                },
            ),
            (
                "Storage",
                "storage {",
                Token {
                    token_type: TokenType::TokenStorage,
                    value: STORAGE_TOKEN_VALUE.to_string(),
                    line_number: 1,
                },
            ),
            (
                "Volume",
                "volume: myvolume",
                Token {
                    token_type: TokenType::TokenVolume,
                    value: "myvolume".to_string(),
                    line_number: 1,
                },
            ),
            (
                "Size",
                "size: 10Gi",
                Token {
                    token_type: TokenType::TokenSize,
                    value: "10Gi".to_string(),
                    line_number: 1,
                },
            ),
            (
                "RightBrace",
                "}",
                Token {
                    token_type: TokenType::TokenRBrace,
                    value: RIGHT_BRACE_VALUE.to_string(),
                    line_number: 1,
                },
            ),
            (
                "Port",
                "port: 8080",
                Token {
                    token_type: TokenType::TokenPort,
                    value: "8080".to_string(),
                    line_number: 1,
                },
            ),
            (
                "TargetPort",
                "targetPort: 80",
                Token {
                    token_type: TokenType::TokenTargetPort,
                    value: "80".to_string(),
                    line_number: 1,
                },
            ),
            (
                "Identifier",
                "someIdentifier",
                Token {
                    token_type: TokenType::TokenIdentifier,
                    value: "someIdentifier".to_string(),
                    line_number: 1,
                },
            ),
        ];

        for (name, input, expected_token) in test_cases {
            let token = lexer.scan_token(input);
            assert_eq!(token, expected_token, "Test case {} failed", name);
        }
    }

    #[test]
    fn test_next_token_with_tokens() {
        let input = "token1\ntoken2";
        let lines = input.lines();
        let tokens = vec![
            Token {
                token_type: TokenType::TokenIdentifier,
                value: "token1".to_string(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::TokenIdentifier,
                value: "token2".to_string(),
                line_number: 2,
            },
        ];
        let mut lexer = Lexer {
            lines,
            tokens,
            pos: 0,
            line_number: 1,
        };

        let token1 = lexer.next_token();
        assert_eq!(token1.token_type, TokenType::TokenIdentifier);
        assert_eq!(token1.value, "token1");

        let token2 = lexer.next_token();
        assert_eq!(token2.token_type, TokenType::TokenIdentifier);
        assert_eq!(token2.value, "token2");

        let eof_token = lexer.next_token();
        assert_eq!(eof_token.token_type, TokenType::TokenEOF);
    }

    #[test]
    fn test_next_token_no_tokens() {
        let input = "";
        let lines = input.lines();
        let tokens = vec![];
        let mut lexer = Lexer {
            lines,
            tokens,
            pos: 0,
            line_number: 1,
        };

        let eof_token = lexer.next_token();
        assert_eq!(eof_token.token_type, TokenType::TokenEOF);
    }

    #[test]
    fn test_parse_string_value_with_colon() {
        let input = "key: value";
        let expected = "value".to_string();
        assert_eq!(parse_string_value(input), expected);
    }

    #[test]
    fn test_parse_string_value_with_quotes() {
        let input = "key: \"value\"";
        let expected = "value".to_string();
        assert_eq!(parse_string_value(input), expected);
    }

    #[test]
    fn test_parse_string_value_with_braces() {
        let input = "key: {value}";
        let expected = "value".to_string();
        assert_eq!(parse_string_value(input), expected);
    }

    #[test]
    fn test_parse_string_value_with_semicolon() {
        let input = "key: value;";
        let expected = "value".to_string();
        assert_eq!(parse_string_value(input), expected);
    }

    #[test]
    fn test_parse_string_value_no_colon() {
        let input = "key value";
        let expected = "".to_string();
        assert_eq!(parse_string_value(input), expected);
    }

    #[test]
    fn test_parse_string_value_empty_string() {
        let input = "";
        let expected = "".to_string();
        assert_eq!(parse_string_value(input), expected);
    }
}
