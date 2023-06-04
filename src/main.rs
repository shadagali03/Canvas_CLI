// use canvas_cli::{account_info, get_courses};
use canvas_cli::Config;
use dotenv::dotenv;
use std::env;
use std::process;

fn main() {
    dotenv().ok();
    let args = env::args().collect::<Vec<String>>();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = canvas_cli::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    // let auth_token = env::var("CANVAS_AUTH_TOKEN").expect("AUTH_TOKEN not set");
    // account_info(&auth_token).unwrap();
    // get_courses(&auth_token).unwrap();
}
