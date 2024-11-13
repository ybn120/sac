use std::{env, process};
use sac::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 1 {
        eprintln!("[ERROR] Usage : ./sac program.bf");
        eprintln!("[ERROR] No program provided !");
        process::exit(1);
    }

    let program_path = &args[1];

    let mut my_interpreter = Interpreter::new();

    my_interpreter.load_program(program_path);

    my_interpreter.interpret();
}
