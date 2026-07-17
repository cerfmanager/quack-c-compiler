mod code_emitter;
mod icbm;
mod icbm_asm;
mod lexer;
mod parser;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Expected file path")
    }
    let path = &args[1].to_owned();
    let output_path = args[1].trim_end_matches(".c");

    let mut lexed = lexer::lexer(path);
    println!("tokens lexed");
    let parsed = parser::parse_program(&mut lexed);
    println!("tokens parsed");
    let asm_tree = icbm_asm::parse_program(parsed);
    println!("tree made into asm");
    code_emitter::asm_to_quack(asm_tree, output_path);
    println!("code Emitted");
}
