use clap::{Arg, Command};

pub fn parse_args() -> clap::ArgMatches {
    Command::new("Node Express REST API Generator")
        .version("1.0")
        .about("Generates a boilerplate for a Node.js Express REST API")
        .arg(Arg::new("project_name")
            .help("The name of the project")
            .required(true)
            .index(1))
        .arg(Arg::new("auth")
            .long("auth")
            .help("Include authentication setup with JWT"))
        .arg(Arg::new("mongodb")
            .long("mongodb")
            .help("Include MongoDB setup"))
        .arg(Arg::new("postgresql")
            .long("postgresql")
            .help("Include PostgreSQL setup"))
        .get_matches()
}
