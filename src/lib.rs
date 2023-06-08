extern crate rpassword;
mod api_calls;
mod data;
mod help;
use chrono::prelude::*;
use colored::Colorize;
use data::FileUpload;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use rpassword::read_password;
use std::env;
use std::fs::{canonicalize, File};
use std::io::{self, Write};

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

/*
 - Change code to follow this format, return as an Ok(resp)
pub fn get_weather(cty: &Vec<String>, st: &Vec<String>) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    // some stuff here
    let url = reqwest::Url::parse_with_params(url, &params)?;
    let res: WeatherResponse = blocking::get(url)?.json()?;
    Ok(res)
}

match api_call::get_weather(&input.city, &input.state) {
    Ok(res) => // do some stuff,
    Err(err) => println!("Error: {}", err)
}

 */

// Gets the command line input
pub struct Config {
    pub command: Option<String>,
    pub arguments: Vec<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        Ok(Config {
            command: Some(args[1].clone()), // Gets the first input which should be the command
            arguments: args[2..].to_vec(), // Gets the rest of the inputs which should be the arguments
        })
    }
}

// Will be given a config struct that will have the command and arguments
pub fn run(config: Config) -> Result<(), &'static str> {
    match config.command {
        // Handle the commands using the run function
        Some(command) => match command.as_str() {
            // Handle: canva account
            "account" => {
                if config.arguments.len() == 0 {
                    account_info().expect("Error getting account info");
                } else {
                    return Err("Too many arguments");
                }
            }
            // Handle: canva courses
            "courses" => {
                if config.arguments.len() == 0 {
                    get_courses().expect("Error getting courses");
                } else {
                    return Err("Too many arguments");
                }
            }

            // Handle: canva assignments <course_id>
            "assignments" => {
                if config.arguments.len() == 1 {
                    get_assignments(&config.arguments[0].parse::<i64>().unwrap())
                        .expect("Error getting assignments");
                } else {
                    return Err("Must provide a course id");
                }
            }

            // Handle canva login
            "login" => {
                if config.arguments.len() == 0 {
                    login().expect("Error logging in");
                } else {
                    return Err("Too many arguments");
                }
            }

            // Handle: canva add <file_path>
            "add" => {
                if config.arguments.len() < 1 {
                    return Err("Must provide a file path");
                }
                add_file(&config.arguments[0]).expect("Error adding file");
            }

            // Handle: canva help
            "help" => println!("{}", help::help_message()),
            _ => println!("{}", "Command not found".red()),
        },
        None => println!("{}", "Must Enter A Command!".red()),
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
pub async fn account_info() -> Result<(), Box<dyn std::error::Error>> {
    // Will abstract this later to just be an api call that takes three params (api_path, data structure that JSON will be mapped to))
    let api_path = format!("{}/api/v1/users/self", env::var("SCHOOL_BASE_URL").unwrap());
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", env::var("CANVAS_AUTH_TOKEN").unwrap())
            .parse()
            .unwrap(),
    );
    let resp = reqwest::Client::new()
        .get(api_path.as_str())
        .headers(headers)
        .send()
        .await?;

    let account_info = resp.json::<data::Account>().await?;
    println!("Coming from Lib and response data: {:#?}", account_info);
    Ok(())
}

/*
function: courses
Description: This function will allow the user to see what courses they are enrolled in
Parameters: auth_token -> but not actually required by user
Return: Result<(), Box<dyn Error>>
 */
#[tokio::main]
pub async fn get_courses() -> Result<(), Box<dyn std::error::Error>> {
    let api_path = format!("{}/api/v1/courses", env::var("SCHOOL_BASE_URL").unwrap());
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", env::var("CANVAS_AUTH_TOKEN").unwrap())
            .parse()
            .unwrap(),
    );
    let resp = reqwest::Client::new()
        .get(api_path.as_str())
        .headers(headers)
        .send()
        .await?;
    // println!("Coming from Lib and response data: {:#?}", resp);
    let user_courses = resp.json::<Vec<data::Course>>().await?;
    let mut valid_courses: Vec<data::ValidCourse> = Vec::new();

    valid_courses.extend(user_courses.iter().filter_map(|course| {
        match (&course.name, &course.course_code) {
            (Some(name), Some(code)) => Some(data::ValidCourse::new(
                name.clone(),
                code.clone(),
                course.id,
            )),
            _ => None,
        }
    }));

    println!(
        "{0: <25} {1: <50} {2: <10}",
        "Course Code".blue(),
        "Course Name".blue(),
        "Course ID".blue()
    );

    for course in valid_courses.iter() {
        println!(
            "{0: <25} {1: <50} {2: <10}",
            course.name,
            course.course_code,
            course.id.to_string().green()
        );
    }
    Ok(())
}

/*
function: assignments
Description: Will return all the assignments within a course
Paramters: course_id
 */
#[tokio::main]
pub async fn get_assignments(course_id: &i64) -> Result<(), Box<dyn std::error::Error>> {
    let api_path = format!(
        "{}/api/v1/courses/{}/assignments",
        env::var("SCHOOL_BASE_URL").unwrap(),
        course_id
    );
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", env::var("CANVAS_AUTH_TOKEN").unwrap())
            .parse()
            .unwrap(),
    );
    let resp = reqwest::Client::new()
        .get(api_path.as_str())
        .headers(headers)
        .send()
        .await?;

    let course_assignments = resp.json::<Vec<data::Assignment>>().await?;
    let mut valid_assignments: Vec<data::ValidAssignment> = Vec::new();

    valid_assignments.extend(course_assignments.iter().filter_map(|assignment| {
        assignment.due_at.as_ref().and_then(|due_date| {
            let new_date = DateTime::parse_from_rfc3339(due_date)
                .unwrap()
                .format("%m-%d-%Y")
                .to_string();
            assignment.name.as_ref().and_then(|name| {
                assignment
                    .id
                    .map(|id| data::ValidAssignment::new(name.clone(), id, new_date.clone()))
            })
        })
    }));

    println!(
        "{0: <40} {1: <20} {2: <10}",
        "Assignment Name".blue(),
        "Due Date".blue(),
        "Assignment ID".blue()
    );

    for assignment in valid_assignments.iter() {
        println!(
            "{0: <40} {1: <20} {2: <10}",
            assignment.name,
            assignment.due_at,
            assignment.id.to_string().green()
        );
    }
    Ok(())
}
/*
function: canva add <file_path>
Description: This function will allow the user to submit an assignment
Parameters: course_id, assignment_id, file_path
Return: Result<(FileUpload), Box<dyn Error>>

- This will be step 1 in uploading a file to canvas.
- The FileUpload struct will be used to store the response from the API call

These are the endpoints that will be used for this function
https://sit.instructure.com/api/v1/users/self/files
 */
#[tokio::main]
pub async fn add_file(file_path: &String) -> Result<data::FileUpload, Box<dyn std::error::Error>> {
    let full_file_path = &canonicalize(file_path).unwrap();
    let split_path: Vec<&str> = full_file_path.to_str().unwrap().split("/").collect();
    let parent_path = &split_path[0..split_path.len() - 1].join("/");
    let file_name = &split_path[split_path.len() - 1];
    // let parent_path = split_path
    println!(
        "parent path: {:?} and child path: {:?}",
        parent_path, file_name
    );
    let temp: FileUpload = data::FileUpload::new(
        "t".to_string(),
        "a".to_string(),
        "t".to_string(),
        "a".to_string(),
    );
    Ok(temp)
}

// Helper function for login to write the user info to the .env file
fn write_to_env(auth_token: &String, school_url: &String) {
    let path = std::path::Path::new(".env");
    let display = path.display();

    let mut env_file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    match env_file.write_all(format!("CANVAS_AUTH_TOKEN={}\n", auth_token).as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => (),
    }
    match env_file.write_all(format!("SCHOOL_BASE_URL={}\n", school_url).as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => (),
    }
    println!("\n{}", "Successfully logged in!".green())
}

// Change this function to take 0 parameters and then prompt the user for the auth token and school name -> censor the auth token using
/*
function: login
Description: This function will allow the user to login to their canvas account
Parameters: auth_token
Return: Result<(), Box<dyn Error>>
 */
#[tokio::main]
pub async fn login() -> Result<(), Box<dyn std::error::Error>> {
    // Get environment path where auth and school url will be stored

    let mut school_url = String::new();
    print!("Enter Canvas School URL: ");
    io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut school_url).unwrap();
    print!("\nEnter Canvas Auth Token: ");
    std::io::stdout().flush().unwrap();
    let auth_token = read_password().unwrap();
    // Need to use the variables as the .env file is not flushed and will cause errors
    let api_path = format!("{}/api/v1/courses", school_url);
    let mut headers = HeaderMap::new();

    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", auth_token).parse().unwrap(),
    );
    let resp = reqwest::Client::new()
        .get(api_path.as_str())
        .headers(headers)
        .send()
        .await;

    match resp {
        Ok(_) => write_to_env(&auth_token, &school_url),
        Err(_) => println!("{}", "Error Logging in! Try again".red()),
    }

    Ok(())
}
