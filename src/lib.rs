extern crate rpassword;
extern crate serde_json;
use data::{CommitData, UploadData};
use reqwest::multipart;
mod api_calls;
mod data;
mod help;
use chrono::prelude::*;
use colored::Colorize;
use reqwest::header::{HeaderMap, AUTHORIZATION};
use rpassword::read_password;
use std::env;
use std::fs::{canonicalize, metadata, File};
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
                    print_account_info(fetch_account_info().expect("Error getting account info"));
                } else {
                    return Err("Too many arguments");
                }
            }
            // Handle: canva courses
            "courses" => {
                if config.arguments.len() == 0 {
                    print_courses(fetch_courses().expect("Error getting courses"));
                } else {
                    return Err("Too many arguments");
                }
            }

            // Handle: canva assignments <course_id>
            "assignments" => {
                if config.arguments.len() == 1 {
                    print_assignments(
                        fetch_assignments(&config.arguments[0].parse::<i64>().unwrap())
                            .expect("Error getting assignments"),
                    );
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

            // Handle: canva commit
            "commit" => {
                if config.arguments.len() > 0 {
                    return Err("Too many arguments");
                }
                commit_file().expect("Error committing file");
            }

            // Handle: canva submit
            "submit" => {
                if config.arguments.len() != 2 {
                    return Err("Must provide a course and assignment id");
                }
                submit_file(
                    &config.arguments[0].parse::<i64>().unwrap(),
                    &config.arguments[1].parse::<i64>().unwrap(),
                )
                .expect("Error submitting file");
            }

            // Handle: canva help
            "help" => println!("{}", help::help_message()),
            _ => println!("{}", "Command not found".red()),
        },
        None => println!("{}", "Must Enter A Command!".red()),
    }
    Ok(())
}

#[tokio::main]
async fn call_canvas_api<T>(path: &String) -> Result<T, &'static str>
where
    T: serde::de::DeserializeOwned,
{
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", env::var("CANVAS_AUTH_TOKEN").unwrap())
            .parse()
            .unwrap(),
    );
    let resp = reqwest::Client::new()
        .get(path.as_str())
        .headers(headers)
        .send()
        .await;
    match resp {
        Ok(resp) => {
            if resp.status().is_success() {
                let account_info = resp.json::<T>().await.unwrap();
                return Ok(account_info);
            } else {
                return Err("Error getting account info");
            }
        }
        Err(_) => return Err("Error getting account info"),
    }
}

#[tokio::main]
async fn post_data_api<T>(path: &String, form: reqwest::multipart::Form) -> Result<T, &'static str>
where
    T: serde::de::DeserializeOwned,
{
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", env::var("CANVAS_AUTH_TOKEN").unwrap())
            .parse()
            .unwrap(),
    );
    let resp = reqwest::Client::new()
        .post(path.as_str())
        .headers(headers)
        .multipart(form)
        .send()
        .await;
    match resp {
        Ok(resp) => {
            if resp.status().is_success() {
                let account_info = resp.json::<T>().await.unwrap();
                return Ok(account_info);
            } else {
                return Err("Error getting account info");
            }
        }
        Err(_) => return Err("Error getting account info"),
    }
}
/*
function: account
Description: This function will allow the user to login to their canvas account
Parameters: auth_token
Return: Result<(), Box<dyn Error>>
 */
fn fetch_account_info() -> Result<data::Account, &'static str> {
    // Will abstract this later to just be an api call that takes three params (api_path, data structure that JSON will be mapped to))
    let api_path = format!("{}/api/v1/users/self", env::var("SCHOOL_BASE_URL").unwrap());
    let user_account_information: Result<data::Account, &'static str> = call_canvas_api(&api_path);

    match user_account_information {
        Ok(resp) => Ok(resp),
        Err(_) => Err("Error getting account info"),
    }
}

// TODO: Change Date Created format to be more readable -> will create a function to do this
fn print_account_info(account_info: data::Account) {
    println!("Account Info:");
    println!("Name: {}", account_info.name);
    println!("ID: {}", account_info.id);
    println!("Date Created: {}", account_info.created_at);
}

/*
function: courses
Description: This function will allow the user to see what courses they are enrolled in
Parameters: auth_token -> but not actually required by user
Return: Result<(), Box<dyn Error>>
 */
fn fetch_courses() -> Result<Vec<data::ValidCourse>, &'static str> {
    let api_path = format!("{}/api/v1/courses", env::var("SCHOOL_BASE_URL").unwrap());
    let user_courses: Result<Vec<data::Course>, &'static str> = call_canvas_api(&api_path);

    let mut valid_courses: Vec<data::ValidCourse> = Vec::new();

    valid_courses.extend(user_courses.unwrap().iter().filter_map(|course| {
        match (&course.name, &course.course_code) {
            (Some(name), Some(code)) => Some(data::ValidCourse::new(
                name.clone(),
                code.clone(),
                course.id,
            )),
            _ => None,
        }
    }));
    Ok(valid_courses)
}

fn print_courses(courses: Vec<data::ValidCourse>) {
    println!(
        "{0: <25} {1: <50} {2: <10}",
        "Course Code".blue(),
        "Course Name".blue(),
        "Course ID".blue()
    );

    for course in courses.iter() {
        println!(
            "{0: <25} {1: <50} {2: <10}",
            course.name,
            course.course_code,
            course.id.to_string().green()
        );
    }
}

/*
function: assignments
Description: Will return all the assignments within a course
Paramters: course_id
 */
fn fetch_assignments(course_id: &i64) -> Result<Vec<data::ValidAssignment>, &'static str> {
    let api_path = format!(
        "{}/api/v1/courses/{}/assignments",
        env::var("SCHOOL_BASE_URL").unwrap(),
        course_id
    );
    let course_assignments: Result<Vec<data::Assignment>, &'static str> =
        call_canvas_api(&api_path);

    let ca: Vec<data::Assignment>;
    match course_assignments {
        Ok(resp) => ca = resp,
        Err(_) => return Err("Error getting assignments"),
    }

    let mut valid_assignments: Vec<data::ValidAssignment> = Vec::new();

    valid_assignments.extend(ca.iter().filter_map(|assignment| {
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

    Ok(valid_assignments)
}
fn print_assignments(assignments: Vec<data::ValidAssignment>) {
    println!(
        "{0: <40} {1: <20} {2: <10}",
        "Assignment Name".blue(),
        "Due Date".blue(),
        "Assignment ID".blue()
    );

    for assignment in assignments.iter() {
        println!(
            "{0: <40} {1: <20} {2: <10}",
            assignment.name,
            assignment.due_at,
            assignment.id.to_string().green()
        );
    }
}
/*
function: canva add [<file_path>] -> can be multiple files
Description: This function will allow the user to submit an assignment
Parameters: course_id, assignment_id, file_path
Return: Result<(UploadData), Box<dyn Error>>

- This will be step 1 in uploading a file to canvas.
- The FileUpload struct will be used to store the response from the API call

These are the endpoints that will be used for this function
https://sit.instructure.com/api/v1/users/self/files
 */
fn add_file(file_path: &String) -> Result<data::UploadData, Box<dyn std::error::Error>> {
    let full_file_path = canonicalize(file_path).unwrap();
    let split_path: Vec<&str> = full_file_path.to_str().unwrap().split("/").collect();
    let parent_path = &split_path[0..split_path.len() - 1].join("/");
    let file_name = split_path[split_path.len() - 1];
    let file_size = metadata(&full_file_path)?.len();
    let api_path = format!(
        "{}/api/v1/users/self/files",
        env::var("SCHOOL_BASE_URL").unwrap(),
    );
    // let mut headers = HeaderMap::new();

    let form: reqwest::multipart::Form = multipart::Form::new()
        .text("size", file_size.to_string())
        .text("parent_folder_path", parent_path.clone())
        .text("file", file_name.to_string());

    let file_upload_data: Result<data::FileUpload, &'static str> = post_data_api(&api_path, form);

    let upload_json = data::UploadData::new(
        file_upload_data.unwrap(),
        file_name.to_string(),
        parent_path.to_string(),
    );
    serde_json::to_writer(
        &File::create("src/secrets/.upload_data.json")?,
        &upload_json,
    )?;

    // println!("{:?}", upload_json);

    Ok(upload_json)
}

/*
function: canva commit
Description: This function will commit the file to canvas
Paramters: None
return: Result<(CommitData), Box<dyn Error>>
*/
fn commit_file() -> Result<data::CommitData, Box<dyn std::error::Error>> {
    let file = File::open("src/secrets/.upload_data.json").expect("File could not be read");
    let file_upload_data: UploadData = serde_json::from_reader(file).expect("Error reading file");

    let form: reqwest::multipart::Form = multipart::Form::new()
        .text("parent_folder_path", file_upload_data.parent_path)
        .text(
            "content_type",
            file_upload_data
                .file_data
                .upload_params
                .unwrap()
                .content_type
                .unwrap(),
        )
        .text("file", file_upload_data.file_name);

    let commit_data: Result<data::CommitData, &'static str> =
        post_data_api(&file_upload_data.file_data.upload_url.unwrap(), form);
    serde_json::to_writer(
        &File::create("src/secrets/.commit_data.json")?,
        &commit_data,
    )?;
    Ok(commit_data.unwrap())
}
/*
function: canva submit <course_id> <assignment_id>
Description: This function will submit the file to canvas
Parameters: course_id, assignment_id
Return: Result<(SubmissionData), Box<dyn Error>>
*/
#[tokio::main]
async fn submit_file(
    course_id: &i64,
    assignment_id: &i64,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("src/secrets/.commit_data.json").expect("File could not be read");
    let file_upload_data: CommitData = serde_json::from_reader(file).expect("Error reading file");
    let form: reqwest::multipart::Form = multipart::Form::new()
        .text("submission[submission_type]", "online_upload")
        .text(
            "submission[file_ids][]",
            file_upload_data.id.unwrap().to_string(),
        );

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", env::var("CANVAS_AUTH_TOKEN").unwrap())
            .parse()
            .unwrap(),
    );

    let submission_path = format!(
        "{}/api/v1/courses/{}/assignments/{}/submissions",
        env::var("SCHOOL_BASE_URL").unwrap(),
        course_id,
        assignment_id
    );

    let resp = reqwest::Client::new()
        .post(submission_path)
        .headers(headers)
        .multipart(form)
        .send()
        .await;

    match resp {
        Ok(_) => println!("{}", "File submitted successfully!".green()),
        Err(_) => println!("{}", "Error Logging in! Try again".red()),
    }
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
async fn login() -> Result<(), Box<dyn std::error::Error>> {
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
