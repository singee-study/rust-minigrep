use minigrep::config::Config;
use std::process::exit;

fn main() {
    let config = Config::parse_from_args().unwrap_or_else(|error| {
        eprintln!("{}", error);
        exit(1);
    });

    println!("{:#?}", config);
}
