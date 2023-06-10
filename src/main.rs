use canvas_cli::Config;
use dotenv::dotenv;
use std::env;
use std::process;

fn main() {
    dotenv().ok();
    let args = env::args().collect::<Vec<String>>();
    let configuration = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = canvas_cli::run(configuration) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
