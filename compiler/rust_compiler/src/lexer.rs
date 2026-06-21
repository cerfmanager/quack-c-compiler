use std::fs::File;
use std::io::Read;

#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
    Int,
    Void,
    Return,
    OpenParenthesis,
    CloseParenthesis,
    OpenBrace,
    CloseBrace,
    Semicolon,
    Constant(i64),
    Identifier(String),
    BitWiseComp,
    Negation,
    Decrement,
}

fn lex(buffer: &String) -> Vec<Tokens> {
    let mut tokens: Vec<Tokens> = Vec::new();
    let mut chars = buffer.chars().peekable();
    while let Some(&c) = chars.peek() {
        match c {
            '(' => {
                chars.next();
                tokens.push(Tokens::OpenParenthesis)
            }
            ')' => {
                chars.next();
                tokens.push(Tokens::CloseParenthesis)
            }
            '{' => {
                chars.next();
                tokens.push(Tokens::OpenBrace)
            }
            '}' => {
                chars.next();
                tokens.push(Tokens::CloseBrace)
            }
            ';' => {
                chars.next();
                tokens.push(Tokens::Semicolon)
            }
            '-' => match chars.peek().expect("") {
                '-' => {
                    chars.next();
                    chars.next();
                    tokens.push(Tokens::Decrement)
                }
                _ => {
                    chars.next();
                    tokens.push(Tokens::Negation)
                }
            },
            '~' => {
                chars.next();
                tokens.push(Tokens::BitWiseComp)
            }

            ' ' | '\n' | '\t' => {
                chars.next();
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut word = String::new();
                while let Some(c) = chars.next_if(|x| x.is_alphanumeric() || c == '_') {
                    word.push(c);
                }
                let token = match word.as_str() {
                    "int" => Tokens::Int,
                    "return" => Tokens::Return,
                    "void" => Tokens::Void,
                    _ => Tokens::Identifier(word),
                };
                tokens.push(token);
            }

            '0'..='9' => {
                let mut number = String::new();
                while let Some(c) = chars.next_if(|x| x.is_ascii_digit()) {
                    number.push(c);
                }
                let constant = Tokens::Constant(number.parse().unwrap());
                tokens.push(constant);
            }

            _ => panic!("Unknown token:{}", c),
        }
    }
    return tokens;
}
fn read_file(path: &String) -> String {
    let path = &path;
    let mut file = match File::open(path) {
        Ok(v) => v,
        Err(e) => panic!("{}", e),
    };
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    return buffer;
}

pub fn lexer(path: &str) -> Vec<Tokens> {
    let buffer = { read_file(&path.to_string()) };

    let tokens = lex(&buffer);

    println!("{:?}", tokens);

    return tokens;
}
