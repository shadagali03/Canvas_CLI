/*
These are the command Canva commands in various situations:
        Managing your account
            account                           Get account information
            courses                           Get courses
            login <auth_token> <school_base_url>Login to your account
        Interacting with files
            add <course_id>                   Add files to a course
            commit <message>                  Commit files to a course
            submit                            Submit files to a course

 */
pub fn help_message() -> String {
    let message = "

Welcome to Canva CLI! This is an application that allows you to interact with your Canvas account from the command line.

Usage: canvas[EXE] [COMMAND] [OPTIONS]

These are the command Canva commands in various situations:
Managing your account
    account                         Get account information
    courses                         Get courses
    assignments <course_id>         Get assignments for a course
    login                           Login to your account
Interacting with files
    add <course_id>                 Add files to a course
    commit <message>                Commit files to a course
    submit                          Submit files to a course
    ";
    return message.to_string();
}
