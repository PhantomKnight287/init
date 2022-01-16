use reqwest;
use std::fs::{create_dir, write};
use std::{env, io, process::Command};

#[allow(non_snake_case)]
#[tokio::main]
async fn main() {
    println!("Welcome TO Phantom Project Creator");
    println!("Enter Project To Create: \n \t 1. NodeJS ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input: u32 = input.trim().parse().expect("Please type a number!");
    match input {
        1 => {
            println!("Creating A New Nodejs project.");
            println!("Enter Project Name: ");
            let mut project_name = String::new();
            io::stdin()
                .read_line(&mut project_name)
                .expect("Failed to read line");
            let project_name = project_name.trim();
            println!("Creating Project Directory: {}", project_name);
            create_dir(project_name).expect("Failed to create directory");
            env::set_current_dir(project_name).expect("Failed to change directory");
            Command::new("sh")
                .args(["-c", "npm init -y"])
                .output()
                .expect("Failed to execute command");
            Command::new("touch")
                .arg("index.js")
                .output()
                .expect("Failed to create file");
            println!("Creating .gitiignore file");
            Command::new("touch")
                .arg(".gitignore")
                .output()
                .expect("Failed to create file");
            let mut isGitNeeded = String::new();
            println!("Do you want to add a git repository? (y/n)");
            io::stdin()
                .read_line(&mut isGitNeeded)
                .expect("Failed to read line");
            if isGitNeeded == "y"
                || isGitNeeded == "yes"
                || isGitNeeded == "Y"
                || isGitNeeded == "Yes"
            {
                println!("Adding git repository");
                Command::new("/usr/bin/git")
                    .args(["init", "--bare"])
                    .output()
                    .expect("Failed to execute command");
                Command::new("/usr/bin/git")
                    .args(["add", "."])
                    .output()
                    .expect("Failed to execute command");
                Command::new("git")
                    .args(["commit", "-m", "Initial Commit"])
                    .output()
                    .expect("Failed to execute command");
                let resp = reqwest::get(
                        "https://raw.githubusercontent.com/gitignore/Node.gitignore/master/Node.gitignore",
                    )
                    .await
                    .expect("Failed to get file");
                write(
                    "./.gitignore",
                    resp.text().await.expect("Failed to read file"),
                )
                .expect("Failed to write file");
            }
            println!("Project Created Successfully!");
        }
        _ => {
            println!("Invalid Option");
        }
    }
}
