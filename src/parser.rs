use node::{BinOp, Node, NodeKind};
use token::*;
use lexer::Lexer;

#[derive(Clone, Debug, PartialEq)]
pub struct Parser<'a> {
    lexer: &'a Lexer,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a Lexer) -> Parser<'a> {
        Parser { lexer: lexer }
    }
}

// impl<'a> Parser<'a> {
//     pub fn get_node(&mut self) -> Result<Node, ()> {
//         match self.lexer.read_token()?.kind {
//             TokenKind::Identifier(name) if name == "if" => unimplemented!(),
//             _ => self.read_expr(),
//         }
//     }
// }
//
// impl<'a> Parser<'a> {
//     pub fn read_expr(&mut self) -> Result<Node, ()> {
//         macro_rules! assign { ($lhs:expr, $rhs:expr, $pos:expr) => (
//            Node::new(
//                 NodeKind::BinaryOp(
//                     Box::new($lhs),
//                     Box::new($rhs),
//                     BinOp::Assign),
//                 $pos
//             ))
//         }
//
//         let mut lhs = self.read_assign()?;
//         while Ok(tok) = self.lexer.read_token() {
//             match tok.kind {
//                 TokenKind::Symbol(Symbol::Assign) => {}
//             }
//         }
//         Ok(lhs)
//     }
// }
