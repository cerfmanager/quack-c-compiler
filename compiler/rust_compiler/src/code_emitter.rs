use crate::icbm_asm;

use std::fs::File;
use std::io;
use std::io::Write;

pub fn asm_to_quack(program: icbm_asm::Program, file: &str) {
    let mut lines: Vec<String> = Vec::new();
    emit_function(&program.function, &mut lines);
    write_to_file(file, &lines).expect("failed to write to file");
}

fn emit_function(function: &icbm_asm::Function, lines: &mut Vec<String>) {
    let is_entry = function.identifier == "main" || function.identifier == "_start";
    lines.push(format!("{}:\n", function.identifier));
    emit_prologue(lines);
    for instruction in &function.body {
        emit_instruction(instruction, lines);
    }
    emit_return(lines, is_entry);
}

fn emit_instruction(instruction: &icbm_asm::Instructions, lines: &mut Vec<String>) {
    match instruction {
        icbm_asm::Instructions::Mov(src, dst) => {
            match src {
                icbm_asm::Operand::Imm(val) => match dst {
                    icbm_asm::Operand::Reg(d_reg) => {
                        let reg = register_to_string(*d_reg);
                        lines.push(format!("irmovd ${val}, {reg}\n"));
                    }

                    icbm_asm::Operand::Stack(d_off) => {
                        emit_stack_offset(lines, *d_off);
                        lines.push(format!("irmovd ${val}, r5\n"));
                        lines.push(format!("rrmmovd r5, r4\n"))
                    }

                    _ => {
                        panic!("dst cannot be and imm")
                    }
                },

                icbm_asm::Operand::Stack(offset) => match dst {
                    icbm_asm::Operand::Reg(d_reg) => {
                        let reg = register_to_string(*d_reg);
                        emit_stack_offset(lines, *offset);
                        lines.push(format!("rmrmovd {reg}, r4\n"))
                    }
                    icbm_asm::Operand::Stack(offset) => {
                        panic!("dst cannot be another stack offset")
                    }
                    _ => {
                        panic!("dst cannot be and imm or another stack offset")
                    }
                },

                icbm_asm::Operand::Reg(reg) => match dst {
                    icbm_asm::Operand::Reg(d_reg) => {
                        let s_reg = register_to_string(*reg);
                        let d_reg = register_to_string(*d_reg);
                        lines.push(format!("rrmovd {s_reg}, {d_reg}\n"))
                    }

                    icbm_asm::Operand::Stack(d_off) => {
                        let s_reg = register_to_string(*reg);
                        emit_stack_offset(lines, *d_off);
                        lines.push(format!("rrmmovd {s_reg}, r4\n"))
                    }
                    _ => {
                        panic!("dst cannot be and imm")
                    }
                },

                _ => {
                    //this should never happen
                }
            }
        }

        icbm_asm::Instructions::AllocateStack(offset) => {
            lines.push(format!("irmovd ${offset}, r5\n"));
            lines.push(format!("sub r5, r6\n"));
        }

        icbm_asm::Instructions::Unary(uniop, dst) => match uniop {
            icbm_asm::Unary_Operator::Neg => match dst {
                icbm_asm::Operand::Reg(reg) => {
                    let f_register = register_to_string(*reg);
                    lines.push(format!("neg {f_register}\n"))
                }
                icbm_asm::Operand::Stack(offset) => {
                    emit_stack_offset(lines, *offset);
                    lines.push(format!("rmrmovd r3, r4\n"));
                    lines.push(format!("neg r3\n"));
                    lines.push(format!("rrmmovd r3, r4\n"));
                }
                _ => {}
            },

            icbm_asm::Unary_Operator::Not => match dst {
                icbm_asm::Operand::Reg(reg) => {
                    let f_register = register_to_string(*reg);
                    lines.push(format!("bitcomp {f_register}\n"))
                }
                icbm_asm::Operand::Stack(offset) => {
                    emit_stack_offset(lines, *offset);
                    lines.push(format!("rmrmovd r3, r4\n"));
                    lines.push(format!("bitcomp r3\n"));
                    lines.push(format!("rrmmovd r3, r4\n"));
                }
                _ => {}
            },
            icbm_asm::Unary_Operator::Dec => match dst {
                icbm_asm::Operand::Reg(reg) => {
                    let f_register = register_to_string(*reg);
                    lines.push(format!("dec {f_register}\n"))
                }
                icbm_asm::Operand::Stack(offset) => {
                    emit_stack_offset(lines, *offset);
                    lines.push(format!("rmrmovd r3, r4\n"));
                    lines.push(format!("dec r3\n"));
                    lines.push(format!("rrmmovd r3, r4\n"));
                }
                _ => {}
            },
        },

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
        icbm_asm::Reg::r6 => String::from("r6"),
        icbm_asm::Reg::r7 => String::from("r7"),
    }
}

fn emit_prologue(lines: &mut Vec<String>) {
    lines.push("pushw r7\n".to_string());
    lines.push("rrmovd r6, r7\n".to_string());
}

fn emit_epilogue(lines: &mut Vec<String>) {
    lines.push("rrmovd r7, r6\n".to_string());
    lines.push("popw r7\n".to_string());
}

fn emit_return(lines: &mut Vec<String>, is_entry: bool) {
    emit_epilogue(lines);
    if is_entry {
        lines.push("halt\n".to_string());
    } else {
        lines.push("ret\n".to_string());
    }
}

fn emit_stack_offset(lines: &mut Vec<String>, offset: i32) {
    let f_offset = -offset;
    lines.push(format!("irmovd ${f_offset}, r5\n"));
    lines.push(format!("rrmovd r7, r4\n"));
    lines.push(format!("sub r5, r4\n"));
}
