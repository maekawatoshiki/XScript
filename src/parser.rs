use node::{BinOp, Node, NodeKind};
use token::*;
use lexer::Lexer;

use std::ops::Range;

// #[derive(Clone, Debug, PartialEq)]
pub struct Parser<'a> {
    lexer: &'a mut Lexer,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Parser<'a> {
        Parser { lexer: lexer }
    }
}

impl<'a> Parser<'a> {
    pub fn get_node(&mut self) -> Result<Node, ()> {
        match self.lexer.peek()?.kind {
            TokenKind::Identifier(ref name) if name == "if" => unimplemented!(),
            TokenKind::Newline => {
                self.lexer.read_token()?; // skip newline
                self.get_node()
            }
            _ => self.read_expr(),
        }
    }
}

macro_rules! range { ($start:expr, $end:expr) => (Range { start:$start, end:$end }) }

impl<'a> Parser<'a> {
    pub fn read_expr(&mut self) -> Result<Node, ()> {
        self.read_assign()
    }

    pub fn read_assign(&mut self) -> Result<Node, ()> {
        macro_rules! assign { ($lhs:expr, $rhs:expr, $range:expr) => (
           Node::new(
                NodeKind::BinaryOp(
                    Box::new($lhs),
                    Box::new($rhs),
                    BinOp::Assign),
                $range
            ))
        }

        macro_rules! assignx { ($lhs:expr, $op:ident) => ({
            let rhs = self.read_assign()?;
            $lhs = assign!(
                $lhs.clone(),
                Node::new(
                    NodeKind::BinaryOp(
                        Box::new($lhs.clone()),
                        Box::new(rhs.clone()),
                        BinOp::$op,
                    ),
                    range!($lhs.range.start, rhs.range.end)
                ),
                range!($lhs.range.start, rhs.range.end)
            ); })
        }

        let mut lhs = self.read_primary()?;
        while let Ok(tok) = self.lexer.read_token() {
            match tok.kind {
                TokenKind::Symbol(Symbol::Assign) => {
                    let rhs = self.read_assign()?;
                    lhs = assign!(
                        lhs.clone(),
                        rhs.clone(),
                        range!(lhs.range.start, rhs.range.end)
                    );
                }
                TokenKind::Symbol(Symbol::AssignAdd) => assignx!(lhs, Add),
                TokenKind::Symbol(Symbol::AssignSub) => assignx!(lhs, Sub),
                TokenKind::Symbol(Symbol::AssignMul) => assignx!(lhs, Mul),
                TokenKind::Symbol(Symbol::AssignDiv) => assignx!(lhs, Div),
                TokenKind::Symbol(Symbol::AssignMod) => assignx!(lhs, Rem),
                TokenKind::Symbol(Symbol::AssignShl) => assignx!(lhs, Shl),
                TokenKind::Symbol(Symbol::AssignShr) => assignx!(lhs, Shr),
                TokenKind::Symbol(Symbol::AssignAnd) => assignx!(lhs, And),
                TokenKind::Symbol(Symbol::AssignOr) => assignx!(lhs, Or),
                TokenKind::Symbol(Symbol::AssignXor) => assignx!(lhs, Xor),
                _ => {
                    self.lexer.unget(&tok);
                    break;
                }
            }
        }
        Ok(lhs)
    }

    pub fn read_primary(&mut self) -> Result<Node, ()> {
        let tok = self.lexer.read_token()?;
        match tok.kind {
            TokenKind::Int(n) => Ok(Node::new(NodeKind::Int(n), tok.range)),
            TokenKind::Float(f) => Ok(Node::new(NodeKind::Float(f), tok.range)),
            TokenKind::Identifier(name) => Ok(Node::new(NodeKind::Variable(name), tok.range)),
            TokenKind::String(s) => Ok(Node::new(NodeKind::String(s), tok.range)),
            TokenKind::Symbol(sym) => match sym {
                Symbol::OpeningParen => {
                    let expr = self.read_expr();
                    if !self.lexer.skip_symbol(Symbol::ClosingParen)? {
                        // TODO
                        return Err(());
                    }
                    expr
                }
                _ => Err(()),
            },
            _ => Err(()),
        }
    }
}
