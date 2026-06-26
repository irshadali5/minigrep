use core::str;
use std::env;
use std::error::Error;
use std::fs;

/// Holds the configuration for the minigrep program.
#[derive(Debug)]
pub struct Config {
    /// The string to search for.
    pub query: String,
    /// The path to the file to search in.
    pub file_path: String,
    /// If true, search is case-insensitive.
    pub ignore_case: bool,
}

impl Config {
    /// Builds a `Config` from command line arguments.
    ///
    /// # Errors
    /// Returns `Err` if query or file path arguments are missing.
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // In iterator or vec the 0th index is tool name itself
        // So we need to move next in order to parse the query and file path
        args.next();

        // checking whether the query provide or not if not then we must return error
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get query"),
        };

        // checking whether the file path provide or not if not then we must return error
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get file path"),
        };

        // add ignore case if need
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Searches for lines containing `query` in `contents` (case-sensitive).
///
/// # Examples
/// ```
/// let results = minigrep::search("duct", "safe\nduct tape");
/// assert_eq!(results, vec!["duct tape"]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Searches for lines containing `query` in `contents` (case-insensitive).
///
/// # Examples
/// ```
/// let results = minigrep::search_case_insensitive("rUsT", "Rust\nTrust me.");
/// assert_eq!(results, vec!["Rust", "Trust me."]);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lowercase = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_lowercase))
        .collect()
}

/// Runs the program: reads the file and prints matching lines.
///
/// # Errors
/// Returns `Err` if the file cannot be read.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    }
    Ok(())
}

// Test for search() and search_insensitive() functions
#[cfg(test)]

mod tests {
    use super::*;
    use std::vec;
    #[test]
    fn case_sensitive() {
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
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
