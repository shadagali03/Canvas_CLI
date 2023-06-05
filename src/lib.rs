mod data;
mod help;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use std::env;
use std::fs::File;
use std::io::Write;

/*
Plan for building the Canvas CLI

- begin with creating the authuroize function that will allow the user to login to their canvas account
    - COMMAND: canva login <auth_token>
    - this will store the auth token in a file in the users home directory
- Get the users information from the canvas api
    - COMMAND: canva user
    - this will return the users information
- Add the ability to get the users courses
    - COMMAND: canva courses
    - this will return the users courses
- Add file/files to a course
    - COMMAND: canva add <course_id> <file/files>
    - this will add the file/files to the course
- Commit the files to the course
    - COMMAND: canva commit <course_id> <file/files>
    - this will commit the file/files to the course as well as add a comment
- Submit the files to the course
    - COMMAND: canva submit <course_id> <file/files>
    - this will submit the file/files to the course as well as add a comment
 */
pub struct Config {
    pub command: Option<String>,
    pub arguments: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let _help_message = "usage: canva [-h | --help]\n <command> [<args>]\n\n";
        let command = args[1].clone();
        Ok(Config {
            command: Some(command),
            arguments: args[2..].to_vec(),
        })
    }
}

// For now I can start with using manual token generation, however, I will need to use OATH2 to get the token

// Will be given a config struct that will have the command and arguments
pub fn run(config: Config) -> Result<(), &'static str> {
    match config.command {
        // Handle the commands using the run function
        Some(command) => match command.as_str() {
            // Handle: canva account
            "account" => {
                if config.arguments.len() == 0 {
                    let auth_token = env::var("CANVAS_AUTH_TOKEN").expect("AUTH_TOKEN not set");
                    account_info(&auth_token).expect("Error getting account info");
                } else {
                    return Err("Too many arguments");
                }
            }
            // Handle: canva courses
            "courses" => {
                if config.arguments.len() == 0 {
                    let auth_token = env::var("CANVAS_AUTH_TOKEN").expect("AUTH_TOKEN not set");
                    get_courses(&auth_token).expect("Error getting courses");
                } else {
                    return Err("Too many arguments");
                }
            }
            // Handle canva login <auth_token> <school_name>
            "login" => {
                let auth_token;
                let school: String;
                if let Some(token) = config.arguments.get(0) {
                    auth_token = token.clone();
                } else {
                    return Err("No auth token provided");
                }
                if let Some(school_name) = config.arguments.get(1) {
                    school = school_name.clone();
                } else {
                    return Err("No school name provided");
                }
                login(&auth_token, &school).expect("Error logging in");
            }
            "help" => println!("{}", help::help_message()),
            _ => println!("Command not found"),
        },
        None => println!("Must Enter A Command!"),
    }
    Ok(())
}

/*
function: account
Description: This function will allow the user to login to their canvas account
Parameters: auth_token
Return: Result<(), Box<dyn Error>>
 */
#[tokio::main]
pub async fn account_info(auth_token: &String) -> Result<(), Box<dyn std::error::Error>> {
    let api_path = format!("{}/api/v1/users/self", env::var("SCHOOL_BASE_URL").unwrap());
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", auth_token).parse().unwrap(),
    );
    let resp = reqwest::Client::new()
        .get(api_path.as_str())
        .headers(headers)
        .send()
        .await?;
    // println!("Coming from Lib and response data: {:#?}", resp);
    let resp_json = resp.json::<data::Account>().await?;
    println!("Coming from Lib and response data: {:#?}", resp_json);
    Ok(())
}

/*
function: courses
Description: This function will allow the user to see what courses they are enrolled in
Parameters: auth_token -> but not actually required by user
Return: Result<(), Box<dyn Error>>
 */
#[tokio::main]
pub async fn get_courses(auth_token: &String) -> Result<(), Box<dyn std::error::Error>> {
    let api_path = format!("{}/api/v1/courses", env::var("SCHOOL_BASE_URL").unwrap());
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", auth_token).parse().unwrap(),
    );
    let resp = reqwest::Client::new()
        .get(api_path.as_str())
        .headers(headers)
        .send()
        .await?;
    // println!("Coming from Lib and response data: {:#?}", resp);
    let resp_json = resp.json::<Vec<data::Course>>().await?;
    println!("Coming from Lib and response data: {:#?}", resp_json);
    Ok(())
}

/*
function: login
Description: This function will allow the user to login to their canvas account
Parameters: auth_token
Return: Result<(), Box<dyn Error>>
 */
pub fn login(auth_token: &String, school_url: &String) -> std::io::Result<()> {
    println!("{}", &auth_token);
    let path = std::path::Path::new(".env");
    let display = path.display();
    let mut env_file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    match env_file.write_all(format!("CANVAS_AUTH_TOKEN={}\n", auth_token).as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    match env_file.write_all(format!("SCHOOL_BASE_URL={}\n", school_url).as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    Ok(())
}
