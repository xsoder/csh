use std::io;
use std::io::Write;
#[derive(Debug)]
enum Token {
    Echo,
    Cat,
    Type,
    Exit,
    Unknown,
}
struct Command {
    status: i32,
    output: String,
}
fn main() {
    let cmd = Command {
        status: 0,
        output: "$".to_string(),
    };
    let input = &mut String::new();
    loop {
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
        let _ = io::stdin().read_line(input).unwrap();
        let input = input.trim();
        let token = match input.split_whitespace().next() {
            Some("echo") => Token::Echo,
            Some("type") => Token::Type,
            Some("cat") => Token::Cat,
            Some("exit") => Token::Exit,
            _ => Token::Unknown,
        };
        accept_command(&cmd, token, input);
    }
}

fn accept_command(_cmd: &Command, token: Token, input: &str) {
    match token {
        Token::Echo => {
            let output = input.strip_prefix("echo").unwrap_or("").trim();
            println!("{}", output);
        }
        Token::Cat => {
            todo!();
        }
        Token::Type => {
            let tp = input.strip_prefix("type").unwrap_or("").trim();
            println!("This command is {}", tp);
        }
        Token::Exit => std::process::exit(0),
        Token::Unknown => {
            println!("Enter a valid input")
        }
    }
}
