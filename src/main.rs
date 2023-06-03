use canvas_cli::{account_info, get_courses};
use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let auth_token = env::var("CANVAS_AUTH_TOKEN").expect("AUTH_TOKEN not set");
    account_info(&auth_token).unwrap();
    get_courses(&auth_token).unwrap();
}
