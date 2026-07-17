use crate::parser::{self};

static mut COUNTER: u32 = 0;

#[derive(Clone)]
pub enum Instructions {
    Return(Val),
    Unary(Unary_Operator, Val, Val),
}

#[derive(Clone)]

pub enum Val {
    Constant(i64),
    Var(String),
}
#[derive(Clone)]
pub enum Unary_Operator {
    Complement,
    Negate,
}

pub struct Function {
    pub identifier: String,
    pub body: Vec<Instructions>,
}

pub struct Program {
    pub function: Function,
}

pub fn parse_program(program: parser::Program) -> Program {
    let function = parse_function(program.function);
    return Program { function };
}

pub fn parse_function(function: parser::Function) -> Function {
    let mut instructions: Vec<Instructions> = Vec::new();
    parse_instructions(function.body, &mut instructions);
    return Function {
        identifier: function.identifier,
        body: instructions,
    };
}

pub fn parse_instructions(instruction: parser::Statement, list: &mut Vec<Instructions>) {
    match instruction {
        parser::Statement::Return(v) => {
            let val = parse_exp(v, list);
            list.push(Instructions::Return(val));
        }
    }
}

pub fn parse_exp(val: parser::Expression, list: &mut Vec<Instructions>) -> Val {
    match val {
        parser::Expression::Constant(v) => return Val::Constant(v),
        parser::Expression::Unary(op, inner) => {
            let src: Val = parse_exp(*inner, list);
            let dst_name = make_temp();
            let dst = Val::Var(dst_name);
            let icbm_op = convert_unop(op);
            list.push(Instructions::Unary(icbm_op, src, dst.clone()));
            return dst;
        }
    }
}

pub fn make_temp() -> String {
    unsafe {
        //TODO: I dont like that this is unsafe find something else
        let val = COUNTER;
        COUNTER += 1;
        return format!("temp.{}", val);
    }
}

pub fn convert_unop(op: parser::UnaryOperator) -> Unary_Operator {
    match op {
        parser::UnaryOperator::Complement => {
            return Unary_Operator::Complement;
        }
        parser::UnaryOperator::Negate => {
            return Unary_Operator::Negate;
        }
        _ => {
            panic!("not covered yet")
        }
    }
}
