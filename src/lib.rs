use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
   pub query: String,
   pub file_path: String,
   pub ignore_case: bool,
}

impl Config {
    pub fn build(args:&[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
        Ok(Config {query, file_path, ignore_case})
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>> {
    let _contents = fs::read_to_string(config.file_path)?;
    
    let _results = if config.ignore_case {
        search_case_insensitive(&config.query, &_contents)
    } else {
        search(&config.query, &_contents)
    };
    
    for line in _results {
        println!("{line}");
    }
    
    Ok(())
}

pub fn search<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {

    let mut results = Vec::new();

    for line in _contents.lines() {
        
        if line.contains(_query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(_query: &str, _contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = _query.to_lowercase();

    for line in _contents.lines() {
        
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
        let contents = "/
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "/
Rust:
safe, fast, productive.
Trust me.";
        assert_eq!(
            vec!["Rust:","Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

