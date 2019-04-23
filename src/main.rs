use std::env;
use std::process;
use hello_rust;
use hello_rust::Config;

fn main() {
    //println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let conf = Config::new(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });    //解开result

    println!("Searching for {}",  conf.query);
    println!("In file {}", conf.filename);

    if let Err(e) = hello_rust::run(conf) {     //match的简写形式

        println!("Application error: {}", e);

        process::exit(1);
    }


}



