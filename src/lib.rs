mod data;
use reqwest::header::{HeaderMap, AUTHORIZATION};

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

// For now I can start with using manual token generation, however, I will need to use OATH2 to get the token

/*
function: login
Description: This function will allow the user to login to their canvas account
Parameters: auth_token
Return: Result<(), Box<dyn Error>>
 */

pub struct Config {
    pub auth_token: Option<String>,
    pub command: Option<String>,
    pub course_id: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        let command = args[1].clone();
        let auth_token = std::env::var("CANVAS_AUTH_TOKEN").ok();
        let course_id = args.get(2).cloned();
        Ok(Config {
            auth_token,
            command: Some(command),
            course_id,
        })
    }
}
#[tokio::main]
pub async fn account_info(auth_token: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", auth_token).parse().unwrap(),
    );
    let resp = reqwest::Client::new()
        .get("https://sit.instructure.com/api/v1/users/self")
        .headers(headers)
        .send()
        .await?;
    // println!("Coming from Lib and response data: {:#?}", resp);
    let resp_json = resp.json::<data::Account>().await?;
    println!("Coming from Lib and response data: {:#?}", resp_json);
    Ok(())
}

#[tokio::main]
pub async fn get_courses(auth_token: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        format!("Bearer {}", auth_token).parse().unwrap(),
    );
    let resp = reqwest::Client::new()
        .get("https://sit.instructure.com/api/v1/courses")
        .headers(headers)
        .send()
        .await?;
    // println!("Coming from Lib and response data: {:#?}", resp);
    let resp_json = resp.json::<Vec<data::Course>>().await?;
    println!("Coming from Lib and response data: {:#?}", resp_json);
    Ok(())
}
