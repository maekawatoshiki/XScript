enum TokenKind {
    Identifier(String),
    Int(i64),
    Float(f64),
    String(String),
    Symbol(Symbol),
}

pub struct Token {
    kind: TokenKind, // The kind of token
    pos: usize,      // The position in code this token appears first
}
