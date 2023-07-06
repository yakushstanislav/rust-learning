use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("query not found in arguments"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("file path not found in arguments"),
        };

        let config = Config {
            query: query,
            file_path: file_path,
        };

        Ok(config)
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &content) {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content.lines().filter(|v| v.contains(query)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn search_positive() {
        let query = "string";

        let content = "\
Hello World!
This is a string!
Rust language
Simple string";

        assert_eq!(
            vec!["This is a string!", "Simple string"],
            search(&query, &content)
        );
    }
}
