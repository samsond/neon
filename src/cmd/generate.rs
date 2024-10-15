use clap::{ArgMatches, Command};

pub fn new_generate_command() -> Command {
    Command::new("generate").about("Generates Kubernetes resources using the Krypton DSL")
}

pub fn execute_generate_command(_matches: &ArgMatches) {
    println!("Generate command executed");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_generate_command() {
        let cmd = new_generate_command();
        let mut app = Command::new("test").subcommand(cmd);
        let matches = app.try_get_matches_from_mut(vec!["test", "generate"]);
        assert!(matches.is_ok(), "Failed to match the 'generate' subcommand");
        let matches = matches.unwrap();
        assert!(
            matches.subcommand_matches("generate").is_some(),
            "'generate' subcommand was not matched"
        );
    }
}
