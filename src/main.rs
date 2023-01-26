// use std::env;
use std::process;

use clap::Parser;

use ugrep::Config;

#[derive(Parser)]
#[command(author, version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    pattern: String,
    path: String,
    #[arg(short, long)]
    case_sensitive: bool,
}

fn main() {
    let cli = Cli::parse();
    // println!("pattern: {}", cli.pattern);
    // println!("path: {}", cli.path);
    // println!("case_sensitive: {:?}", cli.case_sensitive);



    let config = Config::new(cli.pattern, cli.path, cli.case_sensitive);
    print!("Searching for `{}`, in `{}`", config.pattern, config.filename);
    if config.case_sensitive {
        print!(" with case sensisvity");
    }
    println!("\n");

    if let Err(e) = ugrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
