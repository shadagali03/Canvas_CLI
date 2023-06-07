extern crate rpassword;
mod data;
mod help;
use colored::Colorize;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use rpassword::read_password;
use std::env;
use std::fs::File;
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

            "assignments" => {
                if config.arguments.len() == 1 {
                    get_assignments(&config.arguments[0].parse::<i64>().unwrap())
                        .expect("Error getting assignments");
                } else {
                    return Err("Must provide a course id");
                }
            }
            // Handle canva login <auth_token> <school_name>
            "login" => {
                login().expect("Error logging in");
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
pub async fn account_info() -> Result<(), Box<dyn std::error::Error>> {
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

    for course in user_courses.iter() {
        match &course.name {
            Some(name) => match &course.course_code {
                Some(code) => valid_courses.push(data::ValidCourse::new(
                    name.clone(),
                    code.clone(),
                    course.id,
                )),
                None => (),
            },
            None => (),
        }
    }

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
            assignment.name.as_ref().and_then(|name| {
                assignment
                    .id
                    .map(|id| data::ValidAssignment::new(name.clone(), id, due_date.clone()))
            })
        })
    }));

    println!(
        "{0: <25} {1: <50} {2: <10}",
        "Assignment Name".blue(),
        "Due Date".blue(),
        "Assignment ID".blue()
    );

    for assignment in valid_assignments.iter() {
        println!(
            "{0: <25} {1: <50} {2: <10}",
            assignment.name,
            assignment.due_at,
            assignment.id.to_string().green()
        );
    }
    Ok(())
}
#[tokio::main]
pub async fn submit(_config: Config) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
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
