mod input;
mod lexer;

use std::process::ExitCode;
use std::env;

use input::Input;
use lexer::{Lexer, Token};


fn main() -> ExitCode {

    // parse command line args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("no input specified");

        return ExitCode::from(1);
    }

    let file_path: &String = &args[1];

    // create input
    let input: Input = Input::from_file(String::from(file_path));

    // loop over tokens
    let mut lexer: Lexer = Lexer::new(input);

    loop {
        let token: Option<Token> = lexer.next_token();

        match token {
            None => break,
            Some(token) => {
                println!("{}", lexer.pretty_format_token(&token));
            }
        }
    }

    return ExitCode::SUCCESS;
}
