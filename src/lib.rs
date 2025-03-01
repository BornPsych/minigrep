use std::{env, error::Error, fs};

pub struct Config{
    pub query : String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();
        
        Ok(Config{query, file_path, ignore_case})
    }
    
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }else {
        search(&config.query, &contents)
    };

    for line in results{
        println!("{line}");
    }
    Ok(())
}

// By using lifetimes here we can say that we need the output vector to be the slice of the content instead of the query.
// We tell Rust that the data returned by the search function will live as long as the data passed into the search function in the contents argument.

pub fn search<'a>(query:&str, content:&'a str) -> Vec<&'a str>{
   content.
   lines().
   filter(|line| line.contains(query)).
   collect()
}

pub fn search_case_insensitive<'a>(query:&str, content:&'a str) -> Vec<&'a str>{
    content.
    lines().
    filter(|line| line.to_lowercase().
    contains(&query.to_lowercase())).collect()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
    assert_eq!(vec!["safe, fast, productive."], search(query,content));

    }

    #[test]
    fn case_insensetive(){
        let query = "rUsT";
        let content = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
    assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query,content));
    }
}