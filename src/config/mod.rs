use std::env;
use std::env::args;

#[derive(Debug)]
pub struct Config {
    target: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() != 3 {
            return Err(String::from("应当有 2 个参数：要查找的字符串 目标路径"));
        }

        let target = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = match env::var("IGNORE_CASE") {
            Ok(value) => match value.trim() {
                "" | "0" => false,
                _ => true,
            },
            Err(_) => false,
        };

        Ok(Config {
            target,
            file_path,
            ignore_case,
        })
    }

    pub fn parse_from_args() -> Result<Config, String> {
        let args: Vec<String> = args().collect();

        Config::new(&args)
    }
}
