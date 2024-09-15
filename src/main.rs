mod cli;
mod express;
mod auth;
mod mongodb;
mod postgresql;

fn main() {
    let matches = cli::parse_args();

    let project_name = match matches.get_one::<String>("project_name") {
        Some(name) => name,
        None => {
            eprintln!("Error: project_name argument is required.");
            std::process::exit(1);
        }
    };

    express::create_express_app(project_name);

    if matches.contains_id("auth") {
        auth::add_authentication(project_name);
    }
    if matches.contains_id("mongodb") {
        mongodb::add_mongodb(project_name);
    }
    if matches.contains_id("postgresql") {
        postgresql::add_postgresql(project_name);
    }

    println!("Express API project setup complete!");
}