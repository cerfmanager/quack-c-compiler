use crate::parser;

#[derive(Clone)]
pub enum Instructions {
    Mov { src: Expression, dst: Expression },
    Ret,
    Dec { val: Expression },
    Neg { val: Expression },
    Comp { val: Expression },
}
#[derive(Clone)]
pub enum Expression {
    Register(String),
    Immediate(i64),
}

pub struct Function {
    pub identifier: String,
    pub body: Vec<Instructions>,
}

pub struct Program {
    pub function: Function,
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
            parse_expression(v, list);
            list.push(Instructions::Ret);
        }
    }
}

fn parse_expression(expr: parser::Expression, list: &mut Vec<Instructions>) {
    match expr {
        parser::Expression::Constant(v) => {
            let immediate = parse_imm(v);
            let register = parse_register("r0");
            let mov = parse_mov(immediate, register);
            list.push(mov);
        }

        parser::Expression::Unary(operator, exp) => {
            parse_expression(*exp, list);

            match operator {
                parser::UnaryOperator::Complement => {
                    let comp = Instructions::Comp {
                        val: parse_register("r0"),
                    };
                    list.push(comp)
                }
                parser::UnaryOperator::Negate => {
                    let neg = Instructions::Neg {
                        val: parse_register("r0"),
                    };
                    list.push(neg);
                }

                parser::UnaryOperator::Decrement => {
                    let dec = Instructions::Dec {
                        val: parse_register("r0"),
                    };
                    list.push(dec);
                }
            }
        }
    }
}
