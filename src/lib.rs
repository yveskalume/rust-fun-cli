#[cfg(test)]
mod tests;

use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&str> {
        if args.len() < 2 {
            return Err("Missing arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let content = read_file(&config.filename)?;
    for line in search(&config.query, &content) {
        println!("{}", line);
    }
    Ok(())
}

pub fn read_file(file: &str) -> Result<String,Box<dyn Error>> {
    let content = fs::read_to_string(file)?;
    Ok(content)
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}