use clap::Parser;
use std::error::Error;
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
    Rsh,
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
            Some("rsh") => Token::Rsh,
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
                        println!("{}", index);
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
fn accept_command(token: Token, input: &str) {
    match token {
        Token::Ls => {
            todo!();
        }
        Token::Echo => {
            let output = input.strip_prefix("echo").unwrap_or("").trim();
            println!("{}", output);
        }
        Token::Clear => {
            clearscreen::clear().expect("failed to clear screen");
        }
        Token::Rsh => {
            let output = input.strip_prefix("rsh").unwrap_or("").trim();
            let version = 1.0;
            let help = "Help";
            match output {
                "--version" => println!("{}", version),
                "--help" => println!("{}", help),
                _ => panic!(),
            }
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
