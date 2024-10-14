// Declare the nodes module and its submodules
pub mod nodes {
    pub mod node;
    pub mod deployment_node;
    pub mod service_node;
}

// Declare the lexer module
pub mod lexer {
    mod token;
    mod common_literals;
    mod deployment_literals;
}