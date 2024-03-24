use std::{env, process};
use minigrep::Config;

fn main() {
    // ====== Minigrep =========
    // This tool will help you to find lines that contains a given string in a given file
    // Run the project with ```cargo run -- searchString path_to_file.txt

    // let arguments: Vec<String> = env::args().collect();
    
    let config: Config = Config::build(env::args()).unwrap_or_else(|error| {
        eprintln!("Problem in parsing arguments {error}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}