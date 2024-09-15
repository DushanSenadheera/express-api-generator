use std::fs;
use std::process::Command as ShellCommand;

pub fn add_mongodb(project_name: &str) {
    println!("Setting up MongoDB...");

    // Install Mongoose
    let mongo_install = ShellCommand::new("npm")
        .args(&["install", "mongoose"])
        .current_dir(project_name)
        .output()
        .expect("Failed to install Mongoose");

    if mongo_install.status.success() {
        println!("Mongoose installed successfully!");
    } else {
        println!("Failed to install Mongoose!");
    }

    // Modify app.js to add MongoDB connection
    let mongo_connection = r#"
const mongoose = require('mongoose');

mongoose.connect('mongodb://localhost:27017/mydb', {
    useNewUrlParser: true,
    useUnifiedTopology: true
}).then(() => {
    console.log('Connected to MongoDB');
}).catch(err => {
    console.error('Failed to connect to MongoDB', err);
});
"#;

    let mut app_js = fs::read_to_string(format!("{}/app.js", project_name)).expect("Failed to read app.js");
    app_js.push_str(mongo_connection);
    fs::write(format!("{}/app.js", project_name), app_js).expect("Failed to update app.js with MongoDB connection");
}
