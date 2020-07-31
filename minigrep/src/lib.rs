//! # Minigrep
//! 
//! Minigrep is a library containing utilities to search for
//! text from a file

use std::error::Error;
use std::env;
use std::fs;
use std::env::Args;

pub fn run(config: Config) -> Result<(), Box<dyn Error>>
{
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
         
    };

    for line in results
    {
        println!("{}", line);
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    contents.lines().filter(|x| x.contains(&query)).collect()
}

/// Searches contents and returns a vector of lines containing query
/// 
/// # Examples
/// ```
/// let query = "rUsT";
///        let contents = "\
///Rust:
///safe, fast, productive.
///Pick three.
///Trust me.";
///        assert_eq!(vec!["Rust:", "Trust me."], minigrep::search_case_insensitive(query, contents));
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>
{
    let query = query.to_lowercase();
    contents.lines().filter(|x| x.to_lowercase().contains(&query)).collect()
}

impl Config
{
    pub fn new(mut arguments: Args) -> Result<Config, &'static str>
    {
        arguments.next();

        let query = match arguments.next()
        {
            Some(q) => q,
            None => return Err("Didn't get query string")
        };

        let filename = match arguments.next()
        {
            Some(f) => f,
            None => return Err("Didn't get filename string")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config { query, filename, case_sensitive })
    }
}


pub struct Config
{
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_search_case_sensitive()
    {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn test_search_case_insensitive()
    {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));

    }
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = minigrep::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}