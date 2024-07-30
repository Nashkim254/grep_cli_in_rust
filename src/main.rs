use std::env;
use std::process;
use minigrep::Config;
fn main() {
    let conf = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing file: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", conf.query);
    println!("In file {}", conf.file_path);
    if let Err(e) = minigrep::run(conf) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

