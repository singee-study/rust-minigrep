use minigrep::app::run;
use minigrep::config::Config;
use std::process::exit;

fn main() {
    let config = Config::parse_from_args().unwrap_or_else(|error| {
        eprintln!("Error during parse config: {}", error);
        exit(1);
    });

    if let Err(error) = run(config) {
        eprintln!("Error during run application: {}", error);
        exit(1);
    }
}
