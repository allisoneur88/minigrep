use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error parsing arguments: {}", err);
        process::exit(1);
    });

    eprintln!("Searching for: {:?}", config.query);
    eprintln!("In file: {:?}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Applicaton error: {}", e);
        process::exit(1);
    }
}
