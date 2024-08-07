use std::{env, fs, result};
use std::error::Error;

pub struct Config {

    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}
fn config_prase(args: &[String]) -> Config {

    let query = args[1].clone();
    let filename = args[2].clone();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Config { query, filename, case_sensitive }


    
}
impl Config{

    pub fn new(args: &[String]) -> Result<Config, &'static str> {

        if args.len() < 3{

            return Err("not enough arguments");
        }, 

        let query = args[1].clone();

        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query, filename, case_sensitive})
        
    }
}

impl Config{

    pub fn new1(mut args: std::env::Args) -> Result<Confi, &'static> {
        
//标准文档提及有些trait默认实现迭代器，这里的trait是指的类型
        args.next();
        
        let query = match args.next() {

            Some(arg) => arg,
            None => return Err("Dont`get a query string"),
        };

        let filename = match args.next {

            Some(arg) => arg,
            None => return Err("Don`t get a flie name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Config{query, filename, case_insensitive})

    }
}


pub fn run(config: Config) -> Result<(), Box< dyn Error>>{

    let contents = fs::read_to_string(config.filename)?;
    
    let results = if config.case_sensitive{
        search_case_insensitive(&config.query, &contents)
    }else{

        search(&config.query, &contents)
    };

    for line in results {
        
        println!("{}", line);
    }
    Ok(())
}

pub fn search <'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
        
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines(){
        if line.contains(query){

            results.push(line);
        }

    }

    results
}

pub fn search(query: &str, contents: &'a str) -> Vec<&'a str>{

    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    
    let query = query.to_lowercase();
    /*let mut results = Vec::new();
    
    for line in contents.lines(){

        if line.to_lowercase().contains(&query){

            results.push(line);
        }
    }*/
//使用迭代适配器使代码简洁
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}


#[cfg(test)]
mod tests{

    use std::result;

    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";

        let contents = "\
Rust: 
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }


    use super::*;

    #[test]
    fn case_sensitive() {
        
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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
            vec!["Rust:", "Turst me."],
            search_case_insensitive(query, contents)
        )
    }
}

