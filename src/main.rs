use clap::Parser;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(
    name = "rsh",
    version,
    about = "A blazingly fast shell written in rust.",
    author = "xsoder"
)]
struct Args;
#[derive(Debug)]
enum Token {
    Echo,
    Alias,
    Ls,
    Cat,
    Clear,
    Type,
    Exit,
    Unknown,
}

//#[derive(Debug)]
//struct Buffer {
//   value: String,
//   status: i32,
//   aliased: String,
//}

fn main() {
    let _ = Args::parse();
    ctrlc::set_handler(move || {
        print!("\n$ ");
        io::stdout().flush().unwrap();
    })
    .expect("Error setting Ctrl-C handler");
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            println!("Failed to read input");
            continue;
        }
        let input = input.trim();
        let token = match input.split_whitespace().next() {
            Some("echo") => Token::Echo,
            Some("ls") => Token::Ls,
            Some("clear") => Token::Clear,
            Some("type") => Token::Type,
            Some("alias") => Token::Alias,
            Some("cat") => Token::Cat,
            Some("exit") => Token::Exit,
            _ => Token::Unknown,
        };
        accept_command(token, input);
    }
}

fn list_command(input: &str) {
    let path = input.strip_prefix("ls").unwrap_or("").trim();
    match path.find("-") {
        Some(index) => {
            if index == 0 {
                let flag = &path[index + 1..].trim_start();
                let _flags = match flag.split_whitespace().next() {
                    Some("la") => {
			print!("this is from la");
                    }
                    Some("ll") => {
			print!("this is from la");
			todo!();
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

// Accepting Commands
fn accept_command(token: Token, input: &str) {
    match token {
        Token::Ls => {
            list_command(input);
        }
        Token::Alias => {
            alias(input);
        }
        Token::Echo => {
	    let output = input.strip_prefix("echo").unwrap_or("").trim();
	    echo_command(output);
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
                "alias".to_string(),
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

fn alias(input: &str) -> &str {
    let command = input.strip_prefix("alias").unwrap_or("").trim();
    let var: &str;
    match command.find(" ") {
        Some(index) => {
            let strip_command = &command[index + 1..].trim_start();
            let alias_command = &command[..index].trim_end();
            var = alias_command;
	    if var == "" {
		panic!("alias not recored");
	    }
            println!("Stripped command: {}", strip_command);
            println!("Aliased command: {}", alias_command);
        }
        None => {
	    panic!("This is an command invalid type");
        }
    }
    return var;
    }
// Echo command
fn echo_command(output: &str) {
    match output.find("-"){
	Some(index) => {
	    let main_flag = &output[index + 1..].trim_start();
	    println!("{}", main_flag);
	    // TODO: Implementation of flags
	    //let fg = main_flag.chars().nth(1).unwrap();
	    //println!("{}", fg);
	    //let _ = flags.chars().nth(1).unwrap();
		//Some("e") => {
		//    println!("{}",main_flag);
		//},
		//Some("n") => {
		//    println!("{}",main_flag);
		//},
		//Some("E") => {
		//    println!("{}",main_flag);
		//},
		//_ => {
		//    panic!();
		//}
	    }
	None => {
	    println!("{}",output);
	}
    }
    }


//CAT Command
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
