
//! This sample program, `minigrep`, is taken from the Rust Book, 
//! chapter 12 (https://doc.rust-lang.org/book/ch12-00-an-io-project.html) 
//! and chapter 13. Chapter 13 adds iterators and a few extra closures to
//! the sample. 
//! 
//! I am adding comments to the sample to learn more about how comments work in the Rust toolchain.
//! Well that, plus I really like comments. :)  ... when they work {:\
//! 
//! 
//! The Config struct and its new() method are defined first
//! The orchestrating run() function is defined next, followed
//! by two variants to the string-search function
//! 
//!
//! Sample Usage: The main() function (in main.rs) sets up the configuration (environment) 
//! -- the file name and query string, mostly.
//! We then call the run() function to do the work
//! 

use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String, 
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next(); // burn args[0]
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get a query string as a command-line argument"),
        }; // was: args[1].clone() prior to changing the type of args to an Iterator
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("did not get a filename as a command-line argument"),
        };

        // check environment for case-insensitive search flag
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); 
    
        Ok(Config {query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)? ;

    // println!("With text:\n{}", contents);
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    } ;
    
    for line in results {
        println!("hit: {}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /* prior version using mutable intermediate state variables:
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    
    results
    */
    // new version - using iterator adaptor methods and a more functional style:
    contents
        .lines()
        .filter( |line| line.contains(query) )
        .collect()

}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    contents
        .lines()
        .filter( |line| line.to_lowercase().contains(query) )
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], 
            search_case_insensitive(query, contents));
    }

}