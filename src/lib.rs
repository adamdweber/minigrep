use std::error::Error;
use std::{env, fs};


pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}
impl Config{
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next(); //first arg is path for this app
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query string")
        };
        let path = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing file path")
        };
        //let ignore_case = env::var("IGNORE_CASE").is_ok();
        let ignore_case = if let Some(opt) = args.next() {
            match opt.as_str() {
                "cs" => false,
                "ncs" => true,
                _ => return Err("unknown argument"),
            }
        } else {
            env::var("IGNORE_CASE").is_ok()
        };

        Ok(Config {
            query: query,
            file_path: path,
            ignore_case: ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    //println!("contents:\n{contents}");
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );

    }
}