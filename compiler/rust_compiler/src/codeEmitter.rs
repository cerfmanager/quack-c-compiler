use crate::avtAsm;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn asm_to_quack(program: avtAsm::Program, file: &str) {
    let mut lines: Vec<String> = Vec::new();
    emit_function(&program.function, &mut lines);
    write_to_file(file, &lines).expect("failed to write to file");
}

fn emit_function(function: &avtAsm::Function, lines: &mut Vec<String>) {
    lines.push(format!("{}:\n", function.identifier));
    for instruction in &function.body {
        emit_instruction(instruction, lines);
    }
}

fn emit_instruction(instruction: &avtAsm::Instructions, lines: &mut Vec<String>) {
    match instruction {
        avtAsm::Instructions::Mov { src, dst } => {
            let dst_str = match dst {
                avtAsm::Expression::Register(r) => r.clone(),
                _ => panic!("dst must be a register"),
            };
            match src {
                avtAsm::Expression::Immediate(v) => {
                    lines.push(format!("    irmovw ${}, {}\n", v, dst_str));
                }
                avtAsm::Expression::Register(r) => {
                    lines.push(format!("    rrmovw {}, {}\n", r, dst_str));
                }
            }
        }
        avtAsm::Instructions::Ret => {
            lines.push("    halt\n".to_string());
        }
    }
}
pub fn write_to_file(file: &str, lines: &Vec<String>) -> io::Result<()> {
    let mut file = File::create(format!("{}.qasm", file))?;
    for line in lines {
        write!(file, "{}", line)?;
    }
    Ok(())
}
