use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}
impl Config{
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("missing arguments");
        }
        let query = args[1].clone();
        let path = args[2].clone();
        Ok(Config { query: query, file_path: path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("contents:\n{contents}");

    Ok(())
}