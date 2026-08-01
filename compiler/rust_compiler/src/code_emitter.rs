use crate::icbm_asm;
use crate::icbm_asm::Reg::{r0, r3};

use std::fs::File;
use std::io;
use std::io::Write;

pub fn asm_to_quack(program: icbm_asm::Program, file: &str) {
    let mut lines: Vec<String> = Vec::new();
    emit_function(&program.function, &mut lines);
    write_to_file(file, &lines).expect("failed to write to file");
}

fn emit_function(function: &icbm_asm::Function, lines: &mut Vec<String>) {
    lines.push(format!("{}:\n", function.identifier));
    for instruction in &function.body {
        emit_instruction(instruction, lines);
    }
}

fn emit_instruction(instruction: &icbm_asm::Instructions, lines: &mut Vec<String>) {
    match instruction {
        icbm_asm::Instructions::Mov(src, dst) => {
            match src {
                icbm_asm::Operand::Imm(val) => match dst {
                    icbm_asm::Operand::Reg(d_reg) => {
                        let reg = register_to_string(*d_reg);
                        lines.push(format!("irmovd ${val} r{reg}\n"));
                    }

                    icbm_asm::Operand::Stack(d_off) => {
                        lines.push(format!("irmovd ${d_off} r3"));
                        lines.push(format!("rrmovd r5 r2"));
                        lines.push(format!("sub r3 r2"));
                        lines.push(format!("immovd ${val} r2"))
                    }

                    _ => {
                        panic!("dst cannot be and imm")
                    }
                },

                icbm_asm::Operand::Stack(offset) => match dst {
                    icbm_asm::Operand::Reg(d_reg) => {}

                    icbm_asm::Operand::Stack(d_off) => {}
                    _ => {
                        panic!("dst cannot be and imm")
                    }
                },

                icbm_asm::Operand::Reg(reg) => match dst {
                    icbm_asm::Operand::Reg(d_reg) => {}

                    icbm_asm::Operand::Stack(d_off) => {}
                    _ => {
                        panic!("dst cannot be and imm")
                    }
                },

                _ => {
                    //this should never happen
                }
            }
        }

        icbm_asm::Instructions::AllocateStack(offset) => {}

        icbm_asm::Instructions::Unary(uniop, dst) => {}

        icbm_asm::Instructions::Ret => {}
    }
}
pub fn write_to_file(file: &str, lines: &Vec<String>) -> io::Result<()> {
    let mut file = File::create(format!("{}.qasm", file))?;
    for line in lines {
        write!(file, "{}", line)?;
    }
    Ok(())
}

pub fn register_to_string(register: icbm_asm::Reg) -> String {
    match register {
        icbm_asm::Reg::r0 => String::from("r0"),
        icbm_asm::Reg::r1 => String::from("r1"),
        icbm_asm::Reg::r2 => String::from("r2"),
        icbm_asm::Reg::r3 => String::from("r3"),
        icbm_asm::Reg::r4 => String::from("r4"),
        icbm_asm::Reg::r5 => String::from("r5"),
    }
}
