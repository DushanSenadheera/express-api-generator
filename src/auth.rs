use std::fs;
use std::process::Command as ShellCommand;

pub fn add_authentication(project_name: &str) {
    println!("Setting up JWT authentication...");

    // Install necessary packages
    let auth_install = ShellCommand::new("npm")
        .args(&["install", "jsonwebtoken", "bcryptjs"])
        .current_dir(project_name)
        .output()
        .expect("Failed to install authentication packages");

    if auth_install.status.success() {
        println!("JWT and bcrypt installed successfully!");
    } else {
        println!("Failed to install authentication packages!");
    }

    // Add authentication routes to app.js
    let auth_routes = r#"
const jwt = require('jsonwebtoken');
const bcrypt = require('bcryptjs');

app.post('/login', async (req, res) => {
    const { username, password } = req.body;

    // Dummy user for authentication
    const user = { id: 1, username: 'admin', password: 'password' };

    if (username === user.username && await bcrypt.compare(password, user.password)) {
        const token = jwt.sign({ id: user.id }, 'secretkey');
        res.json({ token });
    } else {
        res.status(401).send('Invalid credentials');
    }
});
"#;

    let mut app_js = fs::read_to_string(format!("{}/app.js", project_name)).expect("Failed to read app.js");
    app_js.push_str(auth_routes);
    fs::write(format!("{}/app.js", project_name), app_js).expect("Failed to update app.js with auth routes");
}
