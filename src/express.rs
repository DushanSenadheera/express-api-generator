use std::fs;
use std::process::Command as ShellCommand;

pub fn create_express_app(project_name: &str) {
    println!("Creating Express app...");

    // Create the project folder and initialize npm
    fs::create_dir(project_name).expect("Failed to create project directory");
    let npm_init = ShellCommand::new("npm")
        .args(&["init", "-y"])
        .current_dir(project_name)
        .output()
        .expect("Failed to initialize npm");

    if npm_init.status.success() {
        println!("npm initialized successfully!");
    } else {
        println!("npm init failed!");
    }

    // Install express
    let express_install = ShellCommand::new("npm")
        .args(&["install", "express"])
        .current_dir(project_name)
        .output()
        .expect("Failed to install Express");

    if express_install.status.success() {
        println!("Express installed successfully!");
    } else {
        println!("Failed to install Express!");
    }

    // Create basic files like app.js
    let app_js_content = r#"
const express = require('express');
const app = express();

app.use(express.json());

app.get('/', (req, res) => {
    res.send('Hello World!');
});

const port = process.env.PORT || 3000;
app.listen(port, () => {
    console.log(`Server running on port ${port}`);
});
"#;

    fs::write(format!("{}/app.js", project_name), app_js_content).expect("Failed to write app.js");
}
