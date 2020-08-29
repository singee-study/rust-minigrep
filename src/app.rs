pub mod utils;

pub use self::utils::search;

use crate::config::Config;
use std::error::Error;
use std::fs::read_to_string;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_content = read_to_string(config.file_path)?;

    let search_result = search(&config.target, &file_content);

    for line in search_result {
        println!("{}", line);
    }

    Ok(())
}
