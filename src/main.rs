use std::fs::read_to_string;

use bigbrainfuck::Interpreter;

fn main() {
    let program = read_to_string("test.bbf").unwrap();
    Interpreter::<65536>::new(program.as_bytes().iter().copied()).interpret();
}
