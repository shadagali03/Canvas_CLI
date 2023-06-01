use dotenv::dotenv;
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

/*
function: login
Description: This function will allow the user to login to their canvas account
Parameters: auth_token
Return: Result<(), Box<dyn Error>>
 */

#[tokio::main]
pub async fn login(auth_token: String) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
