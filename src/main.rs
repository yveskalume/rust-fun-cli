use std::env;
use terminal::Config;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|error| {
        println!("Arguments Error: {}",error);
        process::exit(1)
    });
    println!("Searching for '{}'", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = terminal::run(config) {
        println!("Application Error: {}",e)
    };
}
