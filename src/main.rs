use minigrep::Config;
use std::{env, process};

fn main() {
    //let args: Vec<String> = env::args().collect();
    //dbg!(args);

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:  {err}");
        process::exit(1);
    });

    //println!("searching for {}", config.query);
    //println!(" in path {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error:  {e}");
        process::exit(1);
    }
}



