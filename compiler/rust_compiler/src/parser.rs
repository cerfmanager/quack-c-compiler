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
