use std::{ fs, error::Error, env};

pub struct Config {
    pub query_string: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3 {
            return Err("Not enough arguments.");
        }

        let query_string = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

    
        Ok(Config { query_string, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> { // dyn means dynamic
    println!("Searching for {}", config.query_string);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query_string, &contents)
    } else {
        search_case_sensitive(&config.query_string, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query_string: &str, contents: &'a str) -> Vec<&'a str> {

    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {

        if line.contains(query_string) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query_string: &str, contents: &'a str) -> Vec<&'a str> {

    let query_string = query_string.to_lowercase();
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {

        if line.to_lowercase().contains(&query_string) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_case_sensitive() {
        let query_string = "duct";
        let contents = "Rust:\nsafe, fast, productive.\npick three\nDuct.";

        assert_eq!(vec!["safe, fast, productive."], search_case_sensitive(query_string, contents));
    }

    #[test]
    fn test_search_case_insensitive() {
        let query_string = "duct";
        let contents = "Rust:\nsafe, fast, productive.\npick three\nDuct.";

        assert_eq!(vec!["safe, fast, productive.", "Duct."], search_case_insensitive(query_string, contents));
    }
}