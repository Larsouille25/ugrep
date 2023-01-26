use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.pattern, &contents)
    }else {
        search_case_insensitive(&config.pattern, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub pattern: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(pattern: String, filename: String , case_sensitive: bool) -> Config {
        Config { pattern, filename, case_sensitive }
    }
}

pub fn search<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn search_case_insensitive<'a>(pattern: &str, contents: &'a str) -> Vec<&'a str> {
    let pattern = pattern.to_lowercase();
    let mut results: Vec<&'a str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&pattern) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive(){
        let pattern = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents))
    }

    #[test]
    fn case_insensitive(){
        let pattern = "rUsT";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Trust me";

        assert_eq!(
            vec!["Rust:", "Trust me"],
            search_case_insensitive(pattern, contents)
        );
    }
}