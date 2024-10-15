use clap::Command;
use neon::cmd::generate;

fn main() {
    let matches = Command::new("kptn")
        .about("kptn is a CLI for managing Kubernetes resources using the Krypton DSL")
        .subcommand(generate::new_generate_command())
        .get_matches();

    match matches.subcommand() {
        Some(("generate", sub_m)) => generate::execute_generate_command(sub_m),
        _ => eprintln!("Unknown command"),
    }
}
