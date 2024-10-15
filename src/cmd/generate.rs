use crate::lexer::{self, lexer::Lexer};
use clap::{Arg, ArgMatches, Command};

pub fn new_generate_command() -> Command {
    Command::new("generate")
        .about("Generate Kubernetes manifests from a DSL script")
        .arg(
            Arg::new("dsl_file")
                .help("Path to the DSL script")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output_file")
                .help("Path to the output file")
                .short('o')
                .long("output")
                .value_name("FILE"),
        )
        .arg_required_else_help(true)
}

pub fn execute_generate_command(matches: &ArgMatches) {
    let dsl_file_path = matches.get_one::<String>("dsl_file").unwrap();
    let output_file = matches.get_one::<String>("output_file");

    // Read the DSL script from the file
    let raw_input = match std::fs::read_to_string(dsl_file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error reading DSL file: {}", err);
            std::process::exit(1);
        }
    };

    // Trim the input DSL script
    let input = raw_input.trim();

    // Initialize the lexer (assuming you have a lexer module)
    let lexer = Lexer::new(input);

    println!("Generate command executed with DSL file: {}", dsl_file_path);
    // Pretty-print the content of the lexer
    println!("Lexer content: {:#?}", lexer);
    print!("Output file: {:#?}", output_file);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_command_optional_output_arg() {
        let cmd = new_generate_command();
        let mut app = Command::new("test").subcommand(cmd);

        // Test with optional output argument
        let matches = app.try_get_matches_from_mut(vec![
            "test",
            "generate",
            "path/to/dsl",
            "-o",
            "output/file",
        ]);
        assert!(
            matches.is_ok(),
            "Failed to match the 'generate' subcommand with optional output argument"
        );

        let matches = matches.unwrap();
        let sub_matches = matches.subcommand_matches("generate").unwrap();
        assert_eq!(
            sub_matches
                .get_one::<String>("dsl_file")
                .map(|s| s.as_str()),
            Some("path/to/dsl")
        );
        assert_eq!(
            sub_matches
                .get_one::<String>("output_file")
                .map(|s| s.as_str()),
            Some("output/file")
        );
    }

    #[test]
    fn test_new_generate_command() {
        let cmd = new_generate_command();
        let mut app = Command::new("test").subcommand(cmd);
        let matches = app.try_get_matches_from_mut(vec!["test", "generate", "path/to/dsl"]);
        assert!(matches.is_ok(), "Failed to match the 'generate' subcommand");
        let matches = matches.unwrap();
        assert!(
            matches.subcommand_matches("generate").is_some(),
            "'generate' subcommand was not matched"
        );
    }
}
