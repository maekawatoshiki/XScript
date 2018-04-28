use std::ops::Range;

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,     // The kind this token represents
    pub range: Range<usize>, // The range within this token
}

impl Token {
    pub fn new_identifier(name: String, range: Range<usize>) -> Token {
        Token {
            kind: TokenKind::Identifier(name),
            range: range,
        }
    }

    pub fn new_int(n: i64, range: Range<usize>) -> Token {
        Token {
            kind: TokenKind::Int(n),
            range: range,
        }
    }

    pub fn new_float(f: f64, range: Range<usize>) -> Token {
        Token {
            kind: TokenKind::Float(f),
            range: range,
        }
    }

    pub fn new_string(s: String, range: Range<usize>) -> Token {
        Token {
            kind: TokenKind::String(s),
            range: range,
        }
    }

    pub fn new_symbol(symbol: Symbol, range: Range<usize>) -> Token {
        Token {
            kind: TokenKind::Symbol(symbol),
            range: range,
        }
    }

    pub fn new_newline(range: Range<usize>) -> Token {
        Token {
            kind: TokenKind::Newline,
            range: range,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Identifier(String),
    Int(i64),
    Float(f64),
    String(String),
    Symbol(Symbol),
    Newline,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Symbol {
    OpeningParen,
    ClosingParen,
    OpeningBrace,
    ClosingBrace,
    OpeningBoxBracket,
    ClosingBoxBracket,
    Comma,
    Semicolon,
    Colon,
    Point,
    Arrow,
    Inc,
    Dec,
    Add,
    Sub,
    Asterisk,
    Div,
    Mod,
    Not,
    BitwiseNot,
    Shl,
    Shr,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    And,
    Or,
    Xor,
    LAnd,
    LOr,
    Question,
    Assign,
    AssignAdd,
    AssignSub,
    AssignMul,
    AssignDiv,
    AssignMod,
    AssignShl,
    AssignShr,
    AssignAnd,
    AssignOr,
    AssignXor,
    AssignLAnd,
    AssignLOr,
    Hash,
}
