use std::fs;
use std::process::Command as ShellCommand;

pub fn add_postgresql(project_name: &str) {
    println!("Setting up PostgreSQL...");

    // Install pg package
    let pg_install = ShellCommand::new("npm")
        .args(&["install", "pg"])
        .current_dir(project_name)
        .output()
        .expect("Failed to install PostgreSQL");

    if pg_install.status.success() {
        println!("PostgreSQL installed successfully!");
    } else {
        println!("Failed to install PostgreSQL!");
    }

    // Modify app.js to add PostgreSQL connection
    let pg_connection = r#"
const { Pool } = require('pg');
const pool = new Pool({
    user: 'user',
    host: 'localhost',
    database: 'mydb',
    password: 'password',
    port: 5432,
});

pool.query('SELECT NOW()', (err, res) => {
    console.log(err, res);
    pool.end();
});
"#;

    let mut app_js = fs::read_to_string(format!("{}/app.js", project_name)).expect("Failed to read app.js");
    app_js.push_str(pg_connection);
    fs::write(format!("{}/app.js", project_name), app_js).expect("Failed to update app.js with PostgreSQL connection");
}
