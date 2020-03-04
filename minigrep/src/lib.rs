
use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args = std::env::args().collect::<Vec<String>>();

        if args.len() < 3 {
            return Err("Not enough arguments.");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config {
            query,
            filename
        })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let txt = std::fs::read_to_string(&config.filename)?;

    let results = search(&config.query, &txt);

    println!("------ results --------");
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod test {
    use crate::search;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "Rust:\n\
                              safe, fast, productive.\n\
                              Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
