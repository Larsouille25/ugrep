use std::process;

use clap::Parser;
use colored::Colorize;

use ugrep::Config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    // ARGS:
    pattern: String,
    path: String,

    // FLAGS:
    #[arg(short, long, action = clap::ArgAction::SetFalse)]
    case_sensitive: bool,
    #[arg(short, long, action = clap::ArgAction::SetFalse)]
    only_result: bool,
}

fn main() {
    let cli = Cli::parse();
    let config = Config::new(cli.pattern, cli.path, cli.case_sensitive);

    if cli.only_result {
        let mut str = format!(
            "Searching for `{}`, in `{}`",
            config.pattern, config.filename
        );
        if config.case_sensitive {
            str.push_str(" with case insensitivity")
        }

        println!("{} \n", str.italic().bright_black());
    }

    if let Err(e) = ugrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
