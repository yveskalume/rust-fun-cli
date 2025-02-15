#[cfg(test)]
mod tests;

use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config,&str> {
        if args.len() < 3 {
            return Err("Missing arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive: bool =  env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let content = read_file(&config.filename)?;
    let result = if config.case_sensitive {
        search_case_sensitive(&config.query,&content)
    } else {
        search_case_insensitive(&config.query,&content)
    };

    for line in result {
        println!("{}", line);
    }
    Ok(())
}

pub fn read_file(file: &str) -> Result<String,Box<dyn Error>> {
    let content = fs::read_to_string(file)?;
    Ok(content)
}


pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let query = query.to_lowercase();
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line)
        }
    }
    result
}