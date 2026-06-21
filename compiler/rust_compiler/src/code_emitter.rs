use crate::avt_asm;
use std::fmt::format;
use std::fs::File;
use std::io;
use std::io::Write;

pub fn asm_to_quack(program: avt_asm::Program, file: &str) {
    let mut lines: Vec<String> = Vec::new();
    emit_function(&program.function, &mut lines);
    write_to_file(file, &lines).expect("failed to write to file");
}

fn emit_function(function: &avt_asm::Function, lines: &mut Vec<String>) {
    lines.push(format!("{}:\n", function.identifier));
    for instruction in &function.body {
        emit_instruction(instruction, lines);
    }
}

fn emit_instruction(instruction: &avt_asm::Instructions, lines: &mut Vec<String>) {
    match instruction {
        avt_asm::Instructions::Mov { src, dst } => {
            let dst_str = match dst {
                avt_asm::Expression::Register(r) => r.clone(),
                _ => panic!("dst must be a register"),
            };
            match src {
                avt_asm::Expression::Immediate(v) => {
                    lines.push(format!("    irmovw ${}, {}\n", v, dst_str));
                }
                avt_asm::Expression::Register(r) => {
                    lines.push(format!("    rrmovw {}, {}\n", r, dst_str));
                }
            }
        }
        avt_asm::Instructions::Ret => {
            lines.push("    halt\n".to_string());
        }

        avt_asm::Instructions::Comp { val } => match val {
            avt_asm::Expression::Register(v) => {
                lines.push(format!("    two {}\n", v));
            }
            _ => panic!("bitwise should not be used on immidiates "),
        },
        avt_asm::Instructions::Neg { val } => match val {
            avt_asm::Expression::Register(v) => {
                lines.push(format!("    neg {}\n", v));
            }
            _ => panic!("bitwise should not be used on immidiates "),
        },
    }
}
pub fn write_to_file(file: &str, lines: &Vec<String>) -> io::Result<()> {
    let mut file = File::create(format!("{}.qasm", file))?;
    for line in lines {
        write!(file, "{}", line)?;
    }
    Ok(())
}
