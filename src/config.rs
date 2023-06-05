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
