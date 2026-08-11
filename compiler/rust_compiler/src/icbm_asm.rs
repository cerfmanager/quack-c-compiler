use std::collections::HashMap;

use crate::icbm;

#[derive(Clone, Debug)]
pub enum Instructions {
    Mov(Operand, Operand),
    Unary(Unary_Operator, Operand),
    AllocateStack(i32),
    Ret,
}
#[derive(Clone, Copy, Debug)]
pub enum Unary_Operator {
    Neg,
    Not,
    Dec,
}

#[derive(Clone, Debug)]
pub enum Operand {
    Imm(i64),
    Reg(Reg),
    Pseudo(String),
    Stack(i32),
}
#[derive(Clone, Copy, Debug)]
pub enum Reg {
    r0,
    r1,
    r2,
    r3,
    r4,
    r5,
    r6,
    r7,
}

pub struct Function {
    pub identifier: String,
    pub body: Vec<Instructions>,
}

pub struct Program {
    pub function: Function,
}

pub fn parse_program(program: icbm::Program) -> Program {
    let func = parse_function(program.function);
    Program { function: func }
}

pub fn parse_function(function: icbm::Function) -> Function {
    let identifier = function.identifier;

    let mut instructions = parse_instructions(function.body);
    let stack_size = replace_pseudo(&mut instructions);
    // create the stack frame
    instructions.insert(0, Instructions::AllocateStack(-stack_size));
    // fix mov inst when both operands are stack
    let new_inst = inst_fix(&mut instructions);
    print!("{:?}", new_inst);
    Function {
        identifier,
        body: new_inst,
    }
}

pub fn parse_instructions(inst: Vec<icbm::Instructions>) -> Vec<Instructions> {
    let mut instructions: Vec<Instructions> = Vec::new();

    for icbm_inst in inst.iter() {
        match icbm_inst {
            icbm::Instructions::Return(val) => {
                match val.clone() {
                    icbm::Val::Constant(const_val) => {
                        instructions.push(Instructions::Mov(
                            Operand::Imm(const_val),
                            Operand::Reg(Reg::r0),
                        ));
                    }
                    icbm::Val::Var(var_val) => {
                        instructions.push(Instructions::Mov(
                            Operand::Pseudo(var_val),
                            Operand::Reg(Reg::r0),
                        ));
                    }
                }
                instructions.push(Instructions::Ret);
            }

            icbm::Instructions::Unary(operator, src, dst) => {
                match src.clone() {
                    icbm::Val::Constant(const_val) => match dst {
                        icbm::Val::Var(var_val) => {
                            instructions.push(Instructions::Mov(
                                Operand::Imm(const_val),
                                Operand::Pseudo(var_val.clone()),
                            ));
                        }
                        _ => {}
                    },
                    icbm::Val::Var(var_val) => match dst {
                        icbm::Val::Var(var_dest_val) => {
                            instructions.push(Instructions::Mov(
                                Operand::Pseudo(var_val),
                                Operand::Pseudo(var_dest_val.clone()),
                            ));
                        }
                        _ => {}
                    },
                };
                match operator {
                    icbm::Unary_Operator::Negate => match dst {
                        icbm::Val::Var(var_val) => {
                            instructions.push(Instructions::Unary(
                                Unary_Operator::Neg,
                                Operand::Pseudo(var_val.clone()),
                            ));
                        }
                        _ => {}
                    },
                    icbm::Unary_Operator::Complement => match dst {
                        icbm::Val::Var(var_val) => {
                            instructions.push(Instructions::Unary(
                                Unary_Operator::Not,
                                Operand::Pseudo(var_val.clone()),
                            ));
                        }
                        _ => {}
                    },
                    icbm::Unary_Operator::Decrement => match dst {
                        icbm::Val::Var(var_val) => {
                            instructions.push(Instructions::Unary(
                                Unary_Operator::Dec,
                                Operand::Pseudo(var_val.clone()),
                            ));
                        }
                        _ => {}
                    },
                }
            }
        }
    }

    return instructions;
}

pub fn convert_op(op: icbm::Unary_Operator) -> Unary_Operator {
    return match op {
        icbm::Unary_Operator::Complement => Unary_Operator::Not,
        icbm::Unary_Operator::Negate => Unary_Operator::Neg,
        icbm::Unary_Operator::Decrement => Unary_Operator::Dec,
    };
}

pub fn replace_pseudo(instructions: &mut Vec<Instructions>) -> i32 {
    let mut map: HashMap<String, i32> = HashMap::new();
    let mut offset: i32 = 0;
    for index in 0..instructions.len() {
        match instructions[index].clone() {
            Instructions::Mov(src, dst) => {
                let n_src: Operand = match src {
                    Operand::Pseudo(val) => match map.get(&val) {
                        Some(e_offset) => Operand::Stack(*e_offset),
                        None => {
                            offset -= 4;
                            map.insert(val, offset);
                            Operand::Stack(offset)
                        }
                    },
                    _ => src,
                };
                let n_dst: Operand = match dst {
                    Operand::Pseudo(val) => match map.get(&val) {
                        Some(offset) => Operand::Stack(*offset),
                        None => {
                            offset -= 4;
                            map.insert(val, offset);
                            Operand::Stack(offset)
                        }
                    },
                    _ => dst,
                };

                instructions[index] = Instructions::Mov(n_src, n_dst);
            }
            Instructions::Unary(uniop, operand) => match operand {
                Operand::Pseudo(val) => match map.get(&val) {
                    Some(e_offset) => {
                        instructions[index] = Instructions::Unary(uniop, Operand::Stack(*e_offset))
                    }
                    None => {
                        offset -= 4;
                        map.insert(val, offset);
                        instructions[index] = Instructions::Unary(uniop, Operand::Stack(offset))
                    }
                },
                _ => {}
            },
            _ => {}
        }
    }

    offset
}

pub fn inst_fix(instructions: &mut Vec<Instructions>) -> Vec<Instructions> {
    let mut fixed_inst: Vec<Instructions> = Vec::new();

    for index in 0..instructions.len() {
        match instructions[index].clone() {
            Instructions::Mov(src, dst) => match src {
                Operand::Stack(val) => match dst {
                    Operand::Stack(val) => {
                        fixed_inst.push(Instructions::Mov(src, Operand::Reg(Reg::r2)));
                        fixed_inst.push(Instructions::Mov(Operand::Reg(Reg::r2), dst));
                    }
                    _ => {
                        fixed_inst.push(instructions[index].clone());
                    }
                },
                _ => {
                    fixed_inst.push(instructions[index].clone());
                }
            },

            _ => {
                fixed_inst.push(instructions[index].clone());
            }
        }
    }

    return fixed_inst;
}
