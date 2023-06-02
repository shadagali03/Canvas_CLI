use reqwest::header::{HeaderMap, AUTHORIZATION};
use serde::{Deserialize, Serialize};
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    id: i64,
    name: String,
    created_at: String,
    sortable_name: String,
    short_name: String,
    avatar_url: String,
    locale: Option<serde_json::Value>,
    effective_locale: String,
    permissions: Permissions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    can_update_name: bool,
    can_update_avatar: bool,
    limit_parent_app_web_access: bool,
}
#[tokio::main]
pub async fn account_info(auth_token: String) -> Result<(), Box<dyn std::error::Error>> {
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
    let resp_json = resp.json::<Account>().await?;
    println!("Coming from Lib and response data: {:#?}", resp_json);
    Ok(())
}
