use crate::{icbm, lexer::Tokens::Identifier};

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
    parse_function(program.function);
}

pub fn parse_function(function: icbm::Function) -> Function {
    let identifier = function.identifier;

    let instructions = parse_instructions(function.body);
}

pub fn parse_instructions(inst: Vec<icbm::Instructions>) {}
