use crate::lexer::Tokens;

pub enum Expression {
    Constant(i64),
    Unary(UnaryOperator, Box<Expression>),
}

pub enum UnaryOperator {
    Complement,
    Negate,
}

pub enum Statement {
    Return(Expression),
}

pub struct Function {
    pub identifier: String,
    pub body: Statement,
}

pub struct Program {
    pub function: Function,
}

pub fn parse_program(tokens: &mut Vec<Tokens>) -> Program {
    let program = Program {
        function: parse_function(tokens),
    };
    if tokens.len() != 0 {
        panic!("Syntax error, code found after program");
    }
    return program;
}

pub fn parse_function(tokens: &mut Vec<Tokens>) -> Function {
    expect(Tokens::Int, tokens);
    let identifier = expect(Tokens::Identifier("".to_string()), tokens);
    expect(Tokens::OpenParenthesis, tokens);
    expect(Tokens::CloseParenthesis, tokens);
    expect(Tokens::OpenBrace, tokens);
    let body = parse_statement(tokens);
    expect(Tokens::CloseBrace, tokens);
    return Function { identifier, body };
}

//TODO: add the unary things here
pub fn parse_statement(tokens: &mut Vec<Tokens>) -> Statement {
    expect(Tokens::Return, tokens);
    let return_val = parse_exp(tokens);
    expect(Tokens::Semicolon, tokens);
    return Statement::Return(return_val);
}

pub fn parse_exp(tokens: &mut Vec<Tokens>) -> Expression {
    // let next_token = peep_token(tokens);
    // if next_token == Tokens::OpenParenthesis {
    //     let exp_val = parse_exp(tokens);
    //     expect(Tokens::CloseParenthesis, tokens);
    //     return exp_val;
    // } else {
    //     let val = expect(Tokens::Constant(0), tokens);
    //     return Expression::Constant(val.parse().unwrap());
    // }

    match peep_token(tokens) {
        Tokens::CloseParenthesis => {
            let exp_val = parse_exp(tokens);
            expect(Tokens::CloseParenthesis, tokens);
            return exp_val;
        }
        _ => {
            let val = expect(Tokens::Constant(0), tokens);
            return Expression::Constant(val.parse().unwrap());
        }
    }
}

pub fn expect(expected: Tokens, tokens: &mut Vec<Tokens>) -> String {
    let token = take_token(tokens);
    let matches = match (&expected, &token) {
        (Tokens::Identifier(_), Tokens::Identifier(_)) => true,
        (Tokens::Constant(_), Tokens::Constant(_)) => true,
        _ => token == expected,
    };
    if !matches {
        panic!("Syntax error, expected {:?}, found {:?}", expected, token);
    }
    match token {
        Tokens::Identifier(v) => v,
        Tokens::Constant(v) => v.to_string(),
        _ => "".to_string(),
    }
}

pub fn take_token(tokens: &mut Vec<Tokens>) -> Tokens {
    tokens.remove(0)
}

pub fn peep_token(tokens: &mut Vec<Tokens>) -> &Tokens {
    &tokens[0]
}
