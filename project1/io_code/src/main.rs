// ************************************************************************************************
// Vendor           : INWA
// Author           : ning jiu
// Filename         : main 
// Date Created     : 2024.07.29 
// Version          : V1.0
// ************************************************************************************************


use std::env;
use std::process;
use io_code::Config;

fn main() {
    /*let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });*/
    //直接使用env::args返回的迭代器

    let config = Config::new1(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });


    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = io_code::run(config){

        eprintln!("Application error: {}", e);

        process::exit(1)
    }
}





