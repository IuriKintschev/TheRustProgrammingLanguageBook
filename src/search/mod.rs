mod testes;
use std::{env, error, fs, process};

pub fn init() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);

        process::exit(1);
    };
}
struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    fn new(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

fn run(config: Config) -> Result<(), Box<dyn error::Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let result = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in result {
        println!("{}", line)
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut response = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            response.push(line.trim());
        }
    }
    response
}

pub fn search_case_insensitive<'res>(query: &str, contents: &'res str) -> Vec<&'res str> {
    let query = query.to_lowercase();
    let mut response = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            response.push(line.trim());
        }
    }
    response
}
