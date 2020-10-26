use std::{env, error::Error, fs};
use lib::{Config, Query};

pub fn run(Config{ query, filename }: Config) -> Result<Vec<String>, Box<dyn Error>> {
    let current_dir = 
        env::current_dir()?
        .as_path()
        .display()
        .to_string();

    println!("Searching for '{}' in '{}'. Current directory: {}", query, filename.as_str(), current_dir);

    let file_content = fs::read_to_string(filename)?;
    
    let results = 
        search(query, file_content.clone().as_str());
    
    Ok(results)
}

pub mod lib {
    use std::fmt::Display;
    
    pub struct Config {
        pub query: Query,
        pub filename: String,
    }


    impl Config {
        pub fn new(args: &Vec<String>) -> Result<Self, ConfigParsingError> {
            match args.len() - 1 {
                2 => {
                    let query = &args[1];
                    let filename = &args[2];
                    
                    Ok(Config {
                        query: Query::as_case_sensitive(query.clone()),
                        filename: filename.clone(),
                    })
                },
                3 => match args[1].as_str() {
                    "case-insensitive" => {
                        let query = &args[2];
                        let filename = &args[3];
                        
                        Ok(Config {
                            query: Query::as_case_insensitive(query.clone()),
                            filename: filename.clone(),
                        })
                    },
                    first_arg => Err(ConfigParsingError::WrongCaseInsensitiveFlagUsage(first_arg.to_string()))
                },
                _ => Err(ConfigParsingError::TwoArgumentsExpected)
            }
        }
    }

    pub enum ConfigParsingError {
        TwoArgumentsExpected,
        WrongCaseInsensitiveFlagUsage(String)
    }

    pub enum Query {
        CaseSensitive(String),
        CaseInsensitive(String)
    }

    impl Query {
        pub fn as_case_sensitive(query: String) -> Query {
            Query::CaseSensitive(query)
        }

        pub fn as_case_insensitive(query: String) -> Query {
            Query::CaseInsensitive(query.to_lowercase())
        }
    }

    impl Display for Query {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let query = match self {
                Query::CaseSensitive(query) => query.as_str(),
                Query::CaseInsensitive(query) => query.as_str()
            };
            write!(f, "{}", query)
        }
    }

}

fn search(query: Query, contents: & str) -> Vec<String> {
    let mut result = Vec::new();
    for line in contents.lines() {
        match query {
            Query::CaseInsensitive(ref case_insensitive_query) => {
                if line.to_lowercase().contains(case_insensitive_query.as_str()) {
                    result.push(line.to_string());
                }
            }
            Query::CaseSensitive(ref case_sensitive_query) => {
                if line.contains(case_sensitive_query.as_str()) {
                    result.push(line.to_string());
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_case_sensitive_query_when_searching_then_returns_only_one_matching_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
let expected = vec!["safe, fast, productive."];
let actual = search(Query::as_case_sensitive(query.to_string()), contents);
assert_eq!(expected, actual);
}

#[test]
fn given_insensitive_query_when_searching_then_returns_two_matching_case_insensitive_results() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let expected = vec!["Rust:", "Trust me."];
        let actual = search(Query::as_case_insensitive(query.to_string()), contents);
        assert_eq!(expected, actual);
    }
}