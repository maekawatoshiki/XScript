pub struct Token {
    kind: TokenKind, // The kind of token
    pos: usize,      // The position in code this token appears first
}

impl Token {
    pub fn new_identifier(name: String, pos: usize) -> Token {
        Token {
            kind: TokenKind::Identifier(name),
            pos: pos,
        }
    }

    pub fn new_int(n: i64, pos: usize) -> Token {
        Token {
            kind: TokenKind::Int(n),
            pos: pos,
        }
    }

    pub fn new_float(f: f64, pos: usize) -> Token {
        Token {
            kind: TokenKind::Float(f),
            pos: pos,
        }
    }

    pub fn new_string(s: String, pos: usize) -> Token {
        Token {
            kind: TokenKind::String(s),
            pos: pos,
        }
    }

    pub fn new_symbol(symbol: Symbol, pos: usize) -> Token {
        Token {
            kind: TokenKind::Symbol(symbol),
            pos: pos,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum TokenKind {
    Identifier(String),
    Int(i64),
    Float(f64),
    String(String),
    Symbol(Symbol),
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
    Ampersand,
    Shl,
    Shr,
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Ne,
    Xor,
    Or,
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
    AssignXor,
    AssignOr,
    Hash,
}
