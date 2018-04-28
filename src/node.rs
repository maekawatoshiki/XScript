use std::ops::Range;
use std::boxed::Box;

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,      // The kind this node represents
    pub range: Range<usize>, // The range within this node (for error handler)
}

impl Node {
    pub fn new(kind: NodeKind, range: Range<usize>) -> Node {
        Node {
            kind: kind,
            range: range,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Int(i64),
    Float(f64),
    String(String),
    Variable(String),
    BinaryOp(Box<Node>, Box<Node>, BinOp),
    If(Box<Node>, Box<Node>, Box<Node>),
    Apply(Box<Node>, Vec<Node>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    And,
    Or,
    Xor,
    LAnd,
    LOr,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Shl,
    Shr,
    Assign,
}
