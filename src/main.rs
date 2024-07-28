use std::env;
use std::fs;
use std::process;
use std::error::Error;
fn main() {
    let args: Vec<String> = env::args().collect();
    let conf = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing file: {}", err);
        process::exit(1)
    });

    println!("Searching for {}", conf.query);
    println!("In file {}", conf.file_path);
    if let Err(e) = run(conf) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
 fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(conf.file_path)?;
    println!("With text:\n{contents}");
    Ok(())
}
 struct Config {
    query: String,
    file_path: String,
}
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        return Ok(Config { query, file_path });
    }
}
