use std::{env, fs};
use std::error::Error;

pub fn run(conf: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(conf.file_path)?;
    let results;
    if conf.is_case_sensitive {
        results = search(&conf.query, &contents);
    } else {
        results = search_case_sensitive(&conf.query, &contents);
    }
    for line in results {
        println!("line:{}", line);
    }
    Ok(())
}
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub is_case_sensitive: bool,
}
impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let is_case_sensitive = env::var("CASE_SENSITIVE").is_err();
        return Ok(Config { query, file_path, is_case_sensitive});
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

//passing test test
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
}
