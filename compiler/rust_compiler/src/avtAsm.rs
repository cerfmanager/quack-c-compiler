pub enum Instructions {
    Mov {
        src : Expression,
        dst : Expression,
    },
    Ret,


}

pub enum Expression {
    Register(i64),
    Immediate(i64),
}



pub struct Function {
    identifier: String,
    body: Vec<Instructions>,
}

pub struct Program {
    function: Function,
}


pub fn parseProgram(){}
