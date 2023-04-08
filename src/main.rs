mod input;

use std::process::ExitCode;
use std::env;

use input::Input;


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

    println!("{}", input.content);

    return ExitCode::SUCCESS;
}
