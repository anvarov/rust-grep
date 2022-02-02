use std::env;
use std::process;

use rustgrep::Config;
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing args, {}", err);
        process::exit(1);
    });
    // let args: Vec<String> = env::args().collect();
    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     println!("Problem parsing arguments: {}", err);
    //     process::exit(1)
    // });
    println!("Searching for {} \n", config.query);
    print!("In file {} \n", config.filename);
    if let Err(e) = rustgrep::run(config) {
        println!(" Applicatoin Error {}", e);
        process::exit(1)
    } 
}


