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
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {

     args.next();
     let query = match args.next() {
        Some(arg)=> arg,
        None => return Err("Didnt get a query string")
     } ;
     let file_path = match args.next() {
        Some(arg)=> arg,
        None => return Err("Didnt get a file name ")
     };
        let is_case_sensitive = env::var("CASE_SENSITIVE").is_err();
        return Ok(Config { query, file_path, is_case_sensitive});
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
   contents.lines().filter(|line| line.contains(query)).collect()
}
pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines().filter(|line| line.to_lowercase().contains(&query)).collect()
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
