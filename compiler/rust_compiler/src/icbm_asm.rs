use crate::{icbm};

#[derive(Clone)]
pub enum Instructions {
    Mov(Operand, Operand),
    Unary(Unary_Operator, Operand),
    AllocateStack(i64),
    Ret,
}
#[derive(Clone)]
pub enum Unary_Operator {
    Neg,
    Not,
}

#[derive(Clone)]
pub enum Operand {
    Imm(i64),
    Reg(Reg),
    Pseudo(String),
    Stack(i64),
}
#[derive(Clone)]
pub enum Reg {
    r0,
    r1,
    r2,
    r3,
    r4,
    r5,
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

    let instructions = parse_instructions(function.body);

    Function{identifier, body:instructions}
}

pub fn parse_instructions(inst: Vec<icbm::Instructions>) -> Vec<Instructions> {
    let mut instructions: Vec<Instructions> = Vec::new();

    for icbm_inst in inst.iter() {
        match icbm_inst {
            icbm::Instructions::Return(val) => {
                match val.clone() {
                    icbm::Val::Constant(const_val) => {
                        instructions.push(Instructions::Mov(Operand::Imm(const_val), Operand::Reg(Reg::r0)));
                    }
                    icbm::Val::Var(var_val) => {
                        instructions.push(Instructions::Mov(Operand::Pseudo(var_val), Operand::Reg(Reg::r0)));
                    }
                }
                instructions.push(Instructions::Ret);
            }

            icbm::Instructions::Unary(operator, src, dst) => {
                match src.clone() {
                    icbm::Val::Constant(const_val) => {
                        match dst {
                            icbm::Val::Var(var_val) => {
                                instructions.push(Instructions::Mov(Operand::Imm(const_val), Operand::Pseudo(var_val.clone())));
                            }
                            _ => {}
                        }

                    }
                    icbm::Val::Var(var_val) => {
                         match dst {
                            icbm::Val::Var(var_dest_val) => {
                                instructions.push(Instructions::Mov(Operand::Pseudo(var_val), Operand::Pseudo(var_dest_val.clone())));
                            }
                            _ => {}
                        }

                    }
                };
                match operator {
                    icbm::Unary_Operator::Negate=>{

                        match dst {
                            icbm::Val::Var(var_val) => {
                                instructions.push(Instructions::Unary(Unary_Operator::Neg,Operand::Pseudo(var_val.clone())));
                            },
                            _=>{}
                        }


                    },
                    icbm::Unary_Operator::Complement=>{
                        match dst {
                            icbm::Val::Var(var_val) => {
                                instructions.push(Instructions::Unary(Unary_Operator::Not,Operand::Pseudo(var_val.clone())));
                            },
                            _=>{}
                        }
                    }

                }
            }
        }
    }

    return instructions;
}


pub fn convert_op(op: icbm::Unary_Operator) -> Unary_Operator {
    return match op {
        icbm::Unary_Operator::Complement => {
            Unary_Operator::Not
        }
        icbm::Unary_Operator::Negate => {
            Unary_Operator::Neg
        }
    };
}





pub fn replace_pseudo(instructions : &mut Vec<Instructions>) -> Vec<Instructions> {
    
}
