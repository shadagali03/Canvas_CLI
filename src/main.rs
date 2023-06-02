use canvas_cli::account_info;
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let auth_token = env::var("CANVAS_AUTH_TOKEN").expect("AUTH_TOKEN not set");
    account_info(auth_token).unwrap();
}
