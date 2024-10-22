use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::env;

pub struct Config {
    query : String , 
    fileName : String,
    case_sensitive : String
}

impl Config{
    pub fn new(args : &[String])->Result<Config , &'static str>{
        if args.len() < 3{
            return Err("not enough arguments");}
        Ok(Config{
            query : args[1].clone(),
            fileName : args[2].clone(),
            case_sensitive : args[3].to_lowercase().clone()
        })
    }
}

pub fn search<'a>(query : &str, contents : &'a str) -> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }
    results
}
pub fn search_case_insensitive<'a>(query : &str, contents : &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            results.push(line);
        }
    }
    results
}
pub fn run (config : Config)-> Result<() , Box<dyn Error>>{
    let mut s =  String::new();
    let mut f = File::open(config.fileName)?;
    f.read_to_string(&mut s)?;
    let results = if config.case_sensitive == "true" {
        search(&config.query, &s)
    }
    else{
        search_case_insensitive(&config.query, &s)
    };
    for line in results{
        println!("{}", line);
    }
    // println!("the contents of the file is {}" , s);
    Ok(())
}