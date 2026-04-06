use crate::parser;
pub enum Instructions {
    Mov { src: Expression, dst: Expression },
    Ret,
}

pub enum Expression {
    Register(String),
    Immediate(i64),
}

pub struct Function {
    identifier: String,
    body: Vec<Instructions>,
}

pub struct Program {
    function: Function,
}

pub fn parse_imm(value: i64) -> Expression {
    return Expression::Immediate(value);
}

pub fn parse_register(address: &str) -> Expression {
    return Expression::Register(address.to_string());
}

pub fn parse_mov(src: Expression, dst: Expression) -> Instructions {
    return Instructions::Mov { src, dst };
}

pub fn parse_program(program: parser::Program) -> Program {
    let function = parse_function(program.function);
    return Program { function };
}

pub fn parse_function(function: parser::Function) -> Function {
    let instructions: Vec<Instructions> = parse_instructions(function.body);
    return Function {
        identifier: function.identifier,
        body: instructions,
    };
}

pub fn parse_instructions(instruction: parser::Statement) -> Vec<Instructions> {
    let mut instructions_list: Vec<Instructions> = Vec::new();

    match instruction {
        parser::Statement::Return(v) => match v {
            parser::Expression::Constant(v) => {
                let immediate = parse_imm(v);
                let register = parse_register("r0");
                let mov = parse_mov(immediate, register);
                let return_statement = Instructions::Ret;
                instructions_list.push(mov);
                instructions_list.push(return_statement);
            }
        },
    }

    return instructions_list;
}
