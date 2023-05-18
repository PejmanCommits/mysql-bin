use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    // Prompt user for MySQL root password
    println!("Please enter the MySQL root user's password:");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    password = password.trim().to_string();
    // Log in to MySQL and execute commands
    let output = Command::new("mysql")
        .arg("-u")
        .arg("root")
        .arg(format!("--password={}", password))
        .arg("-e")
        .arg("FLUSH LOGS; RESET MASTER;")
        .output()
        .expect("Failed to execute command");
    if !output.status.success() {
        println!("Error: {}", String::from_utf8_lossy(&output.stderr));
        return;
    }
    // Create configuration file
    let mut file = File::create("/etc/mysql/conf.d/disable_binary_log.cnf")
        .expect("Failed to create file");
    file.write_all(b"[mysqld]\nskip-log-bin")
        .expect("Failed to write to file");
    println!("Configuration file created successfully");
}