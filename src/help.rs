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
    let message = "\nusage: canva <command> [<args>]\nThese are the command Canva commands in various situations:\nManaging your account\n\taccount\t\tGet account information\n\tcourses\t\tGet courses\n\t login <auth_token> <school_base_url>\t\tLogin to your account\nInteracting with files\n\tadd <course_id>\t\tAdd files to a course\n\tcommit <message>\t\tCommit files to a course\n\tsubmit\t\tSubmit files to a course\n";
    return message.to_string();
}
