use clap::Parser;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(
    name = "rsh",
    version,
    about = "A blazingly fast shell",
    author = "xsoder"
)]
struct Args;
#[derive(Debug)]
enum Token {
    Echo,
    Ls,
    Cat,
    Clear,
    Type,
    Exit,
    Unknown,
}
fn main() {
    let _ = Args::parse();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let token = match input.split_whitespace().next() {
            Some("echo") => Token::Echo,
            Some("ls") => Token::Ls,
            Some("clear") => Token::Clear,
            Some("type") => Token::Type,
            Some("cat") => Token::Cat,
            Some("exit") => Token::Exit,
            _ => Token::Unknown,
        };
        accept_command(token, input);
    }
}

fn cat_command(out: String) -> Result<(), Box<dyn Error>> {
    match out.find(">") {
        Some(index) => {
            if index == 0 {
                let file_path = &out[index + 1..].trim_start();
                match file_path.find("<<") {
                    Some(index) => {
                        let end = &file_path[index + 2..].trim_start();
                        let path = &file_path[..index].trim_end();
                        loop {
                            print!("> ");
                            io::stdout().flush().unwrap();
                            let mut input = String::new();
                            let _ = io::stdin().read_line(&mut input).unwrap();
                            let input = input.trim();
                            let mut file = OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(path)
                                .unwrap();
                            if input.contains(end) {
                                break;
                            } else {
                                writeln!(file, "{}", input).unwrap();
                            }
                        }
                    }
                    None => {
                        print!("> ");
                        io::stdout().flush().unwrap();
                        let mut input = String::new();
                        let _ = io::stdin().read_line(&mut input).unwrap();
                        let input = input.trim();
                        _ = fs::write(file_path, input);
                    }
                }
            }
        }
        None => {
            let message: String = fs::read_to_string(out)?;
            println!("{}", message);
        }
    }
    return Ok(());
}
fn list_command(input: &str) {
    let path = input.strip_prefix("ls").unwrap_or("").trim();
    match path.find("-") {
        Some(index) => {
            if index == 0 {
                let flag = &path[index + 1..].trim_start();
                let _flags = match flag.split_whitespace().next() {
                    Some("la") => {
                        todo!()
                    }
                    Some("ll") => {
                        todo!()
                    }
                    _ => panic!(),
                };
            } else {
                println!("Invalid Error");
            }
        }
        None => {
            if path.is_empty() {
                let dirs = fs::read_dir("./").unwrap();
                for dir in dirs {
                    let entry = dir.unwrap();
                    let path = entry.path();
                    let remove_trail = path.strip_prefix("./").unwrap_or(&path);
                    print!("{}\t", remove_trail.display());
                }
            }
        }
    }
    print!("\n")
}
fn accept_command(token: Token, input: &str) {
    match token {
        Token::Ls => {
            list_command(input);
        }
        Token::Echo => {
            let output = input.strip_prefix("echo").unwrap_or("").trim();
            println!("{}", output);
        }
        Token::Clear => {
            clearscreen::clear().expect("failed to clear screen");
        }
        Token::Cat => {
            let out = input.strip_prefix("cat").unwrap_or("").trim();
            let _ = cat_command(out.to_string());
        }
        Token::Type => {
            let out = input.strip_prefix("type").unwrap_or("").trim();
            let commands: &[String] = &[
                "echo".to_string(),
                "cat".to_string(),
                "ls".to_string(),
                "clear".to_string(),
                "type".to_string(),
                "exit".to_string(),
            ];
            for command in commands {
                if command == out {
                    println!("This type is command");
                    break;
                } else {
                    continue;
                }
            }
        }
        Token::Exit => std::process::exit(0),
        Token::Unknown => {
            println!("Enter a valid input")
        }
    }
}
