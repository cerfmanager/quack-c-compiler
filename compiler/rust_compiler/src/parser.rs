use crate::lexer::Tokens;

pub enum Expect_Val {
    Identifier(String),
    Constant(i64),
    Null,
}

pub enum Expression {
    Constant(i64),
}

pub enum Statement {
    Return(Expression),
}

pub struct Function {
    indentifier: String,
    body: Statement,
}

pub struct Program {
    function: Function,
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
    let identifer = expect(Tokens::Identifier("".to_string()), tokens);
    expect(Tokens::OpenParenthesis, tokens);
    expect(Tokens::CloseParenthesis, tokens);
    expect(Tokens::OpenBrace, tokens);
    let body = parse_statement(tokens);
    expect(Tokens::CloseBrace, tokens);
    return Function {
        indentifier: identifer,
        body: body,
    };
}

pub fn parse_statement(tokens: &mut Vec<Tokens>) -> Statement {
    expect(Tokens::Return, tokens);
    let return_val = parse_exp(tokens);
    expect(Tokens::Semicolon, tokens);
    return Statement::Return(return_val);
}

pub fn parse_exp(tokens: &mut Vec<Tokens>) -> Expression {
    let next_token = peep_token(tokens);
    if next_token == Tokens::OpenParenthesis {
        let exp_val = parse_exp(tokens);
        expect(Tokens::CloseBrace, tokens);
        return exp_val;
    } else {
        let val = expect(Tokens::Constant(0), tokens);
        return Expression::Constant(val.parse().unwrap());
    }
}

pub fn expect(expected: Tokens, tokens: &mut Vec<Tokens>) -> String {
    let token = take_token(tokens);
    if token != expected {
        panic! {"Syntax error, expected {:?} , found {:?}",token,expected};
    }
    match token {
        Tokens::Identifier(v) => v.to_string(),
        Tokens::Constant(v) => v.to_string(),
        _ => "".to_string(),
    }
}

pub fn take_token(tokens: &mut Vec<Tokens>) -> Tokens {
    let token = tokens.iter_mut().next().unwrap().to_owned();
    return token;
}

pub fn peep_token(tokens: &mut Vec<Tokens>) -> Tokens {
    let token = tokens.iter().peekable().peek_mut().unwrap().clone();
    return token;
}
