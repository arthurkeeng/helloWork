extern crate helloWork;
use std::env;
use std::process;

use helloWork::{Config, run};

fn main(){
    let args: Vec<String> = env::args().collect();
    let mut s = String::new();
    
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("there was a problem parsing arguments : {}" , err);
        process::exit(1);
    });

    if let Err(e) = run(config){
        eprintln!("Application Error {}", e);
        process::exit(1);
    }

    
}


