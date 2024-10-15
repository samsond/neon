// Declare the nodes module and its submodules
pub mod nodes {
    pub mod deployment_node;
    pub mod node;
    pub mod service_node;
}

// Declare the lexer module
pub mod lexer {
    mod common_literals;
    mod deployment_literals;
    pub mod lexer;
    mod token;
}

pub mod cmd {
    pub mod generate;
}
