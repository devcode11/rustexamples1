use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn from(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Syntax: {} <string to search> <filename> [--case_sensitive]", &args[0]));
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = args.iter().find(|&a| *a == "--case_sensitive").is_some();

       Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = search(&config.query, &contents, config.case_sensitive);

    for line in results.iter() {
        println!("{}", line);
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str, case_sensitive: bool) -> Vec<&'a str> {
    if case_sensitive {
        contents.lines().filter(|&l| l.find(query).is_some()).collect()
    } else {
        let lquery = query.to_lowercase();
        contents.lines().filter(|&l| l.to_lowercase().find(&lquery).is_some()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_match() {
        let query = "hello";
        let contents = "\
Ello boy
How are you doing?
Person: hello";

        assert_eq!(vec!["Person: hello"], search(query, contents, false));
    }

    #[test]
    fn no_match() {
        let query = "helloo";
        let contents = "\
Hello boy
How are you doing?
Person: hello";

        assert_eq!(0, search(query, contents, false).len());
    }


    #[test]
    fn multi_match() {
        let query = "hello";
        let contents = "\
Hello boy
How are you doing?
Person: hello";

        assert_eq!(vec!["Hello boy", "Person: hello"], search(query, contents, false));
    }


    #[test]
    fn one_case_sensitive_match() {
        let query = "Hello";
        let contents = "\
Hello boy
How are you doing?
Person: hello";

        assert_eq!(vec!["Hello boy"], search(query, contents, true));
    }
}
