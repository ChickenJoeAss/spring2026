use std::io;
use std::process::Command;

enum FileOperation {
    List(String),
    Display(String),
    Create(String, String),
    Remove(String),
    Pwd,
}

fn perform_operation(operation: FileOperation) {
    match operation {
        FileOperation::List(path) => {
            Command::new("ls")
                .arg(path)
                .status()
                .expect("Failed to execute ls");
        }

        FileOperation::Display(path) => {
            Command::new("cat")
                .arg(path)
                .status()
                .expect("Failed to execute cat");
        }

        FileOperation::Create(path, content) => {
            let command = format!("echo '{}' > {}", content, path);

            Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");

            println!("File '{}' created successfully.", path);
        }

        FileOperation::Remove(path) => {
            Command::new("rm")
                .arg(&path)
                .status()
                .expect("Failed to remove file");

            println!("File '{}' removed successfully.", path);
        }

        FileOperation::Pwd => {
            Command::new("pwd")
                .status()
                .expect("Failed to execute pwd");
        }
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");

        println!("Enter your choice (0-5):");

        let choice = read_input();

        match choice.as_str() {
            "1" => {
                println!("Enter directory path:");
                let path = read_input();
                perform_operation(FileOperation::List(path));
            }

            "2" => {
                println!("Enter file path:");
                let path = read_input();
                perform_operation(FileOperation::Display(path));
            }

            "3" => {
                println!("Enter file path:");
                let path = read_input();

                println!("Enter content:");
                let content = read_input();

                perform_operation(FileOperation::Create(path, content));
            }

            "4" => {
                println!("Enter file path:");
                let path = read_input();
                perform_operation(FileOperation::Remove(path));
            }

            "5" => {
                perform_operation(FileOperation::Pwd);
            }

            "0" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                println!("Invalid option. Try again.");
            }
        }
    }
}