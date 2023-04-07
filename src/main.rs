use std::process::ExitCode;
use std::env;
use std::fs;

fn main() -> ExitCode {

    // parse command line args
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("no input specified");

        return ExitCode::from(1);
    }

    let file_path: &String = &args[1];

    // read file
    let file_contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("{file_contents}");

    return ExitCode::SUCCESS;
}
