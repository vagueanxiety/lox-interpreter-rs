use lox_interpreter_rs::Interpreter;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::process;

fn main() {
    let mut args = env::args();
    args.next();
    // TODO: make debug mode a commnad line argument
    match args.next() {
        Some(file_path) => run_file(&file_path),
        None => run_prompt(),
    };
}

pub fn run_prompt() {
    println!("Entering REPL...");
    let mut interpreter = Interpreter::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        match interpreter.run(line, io::stdout(), io::stderr(), false) {
            Err(e) => eprintln!("{e}"),
            _ => {}
        };
    }
}

// exit codes are implemented differently from the book for now
// 65: err reading file
// 70: internal err (one of scanning, parsing and runtime error)
pub fn run_file(file_path: &str) {
    let contents = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Problem reading file: {err}");
        process::exit(65);
    });
    let mut interpreter = Interpreter::new();
    interpreter
        .run(contents, io::stdout(), io::stderr(), true)
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(70);
        });
}
