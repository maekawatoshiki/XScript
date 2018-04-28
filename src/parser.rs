use node::{BinOp, Node, NodeKind};
use token::*;
use lexer::Lexer;
use typing::ToType;

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

        let mut lhs = self.read_lor()?;
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

    fn read_lor(&mut self) -> Result<Node, ()> {
        let mut lhs = try!(self.read_land());
        while self.lexer.skip_symbol(Symbol::LOr)? {
            let rhs = try!(self.read_land());
            lhs = Node::new(
                NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::LOr),
                range!(lhs.range.start, rhs.range.end),
            );
        }
        Ok(lhs)
    }

    fn read_land(&mut self) -> Result<Node, ()> {
        let mut lhs = try!(self.read_or());
        while self.lexer.skip_symbol(Symbol::LAnd)? {
            let rhs = try!(self.read_or());
            lhs = Node::new(
                NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::LAnd),
                range!(lhs.range.start, rhs.range.end),
            );
        }
        Ok(lhs)
    }

    fn read_or(&mut self) -> Result<Node, ()> {
        let mut lhs = self.read_xor()?;
        while self.lexer.skip_symbol(Symbol::Or)? {
            let rhs = self.read_xor()?;
            lhs = Node::new(
                NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Or),
                range!(lhs.range.start, rhs.range.end),
            );
        }
        Ok(lhs)
    }

    fn read_xor(&mut self) -> Result<Node, ()> {
        let mut lhs = self.read_and()?;
        while self.lexer.skip_symbol(Symbol::Xor)? {
            let rhs = self.read_and()?;
            lhs = Node::new(
                NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Xor),
                range!(lhs.range.start, rhs.range.end),
            );
        }
        Ok(lhs)
    }

    fn read_and(&mut self) -> Result<Node, ()> {
        let mut lhs = self.read_eq_ne()?;
        while self.lexer.skip_symbol(Symbol::And)? {
            let rhs = self.read_eq_ne()?;
            lhs = Node::new(
                NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::And),
                range!(lhs.range.start, rhs.range.end),
            );
        }
        Ok(lhs)
    }

    fn read_eq_ne(&mut self) -> Result<Node, ()> {
        let mut lhs = self.read_relation()?;
        loop {
            if self.lexer.skip_symbol(Symbol::Eq)? {
                let rhs = self.read_relation()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Eq),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else if self.lexer.skip_symbol(Symbol::Ne)? {
                let rhs = self.read_relation()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Ne),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn read_relation(&mut self) -> Result<Node, ()> {
        let mut lhs = self.read_shl_shr()?;
        loop {
            if self.lexer.skip_symbol(Symbol::Lt)? {
                let rhs = self.read_shl_shr()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Lt),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else if self.lexer.skip_symbol(Symbol::Le)? {
                let rhs = self.read_shl_shr()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Le),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else if self.lexer.skip_symbol(Symbol::Gt)? {
                let rhs = self.read_shl_shr()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Gt),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else if self.lexer.skip_symbol(Symbol::Ge)? {
                let rhs = self.read_shl_shr()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Ge),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn read_shl_shr(&mut self) -> Result<Node, ()> {
        let mut lhs = self.read_add_sub()?;
        loop {
            if self.lexer.skip_symbol(Symbol::Shl)? {
                let rhs = self.read_add_sub()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Shl),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else if self.lexer.skip_symbol(Symbol::Shr)? {
                let rhs = self.read_add_sub()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Shr),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn read_add_sub(&mut self) -> Result<Node, ()> {
        let mut lhs = self.read_mul_div_rem()?;
        loop {
            if self.lexer.skip_symbol(Symbol::Add)? {
                let rhs = self.read_mul_div_rem()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Add),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else if self.lexer.skip_symbol(Symbol::Sub)? {
                let rhs = self.read_mul_div_rem()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Sub),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn read_mul_div_rem(&mut self) -> Result<Node, ()> {
        let mut lhs = self.read_call()?;
        loop {
            if self.lexer.skip_symbol(Symbol::Asterisk)? {
                let rhs = self.read_call()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Mul),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else if self.lexer.skip_symbol(Symbol::Div)? {
                let rhs = self.read_call()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Div),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else if self.lexer.skip_symbol(Symbol::Mod)? {
                let rhs = self.read_call()?;
                lhs = Node::new(
                    NodeKind::BinaryOp(Box::new(lhs.clone()), Box::new(rhs.clone()), BinOp::Rem),
                    range!(lhs.range.start, rhs.range.end),
                );
            } else {
                break;
            }
        }
        Ok(lhs)
    }

    fn read_call(&mut self) -> Result<Node, ()> {
        let f = self.read_primary()?;
        let f_start = f.range.start;
        let mut args = vec![];
        while let Ok(arg) = self.read_primary() {
            args.push(arg);
        }
        if args.is_empty() {
            Ok(f)
        } else {
            let args_end = args.last().unwrap().range.end;
            Ok(Node::new(
                NodeKind::Apply(Box::new(f), args),
                range!(f_start, args_end),
            ))
        }
    }

    fn read_primary(&mut self) -> Result<Node, ()> {
        let tok = self.lexer.read_token()?;
        match tok.kind {
            TokenKind::Int(n) => Ok(Node::new(NodeKind::Int(n), tok.range)),
            TokenKind::Float(f) => Ok(Node::new(NodeKind::Float(f), tok.range)),
            TokenKind::Identifier(name) => self.read_variable(name, tok.range),
            TokenKind::String(s) => Ok(Node::new(NodeKind::String(s), tok.range)),
            TokenKind::Symbol(ref sym) => match sym {
                &Symbol::OpeningParen => {
                    let expr = self.read_expr();
                    if !self.lexer.skip_symbol(Symbol::ClosingParen)? {
                        // TODO
                        return Err(());
                    }
                    expr
                }
                _ => {
                    self.lexer.unget(&tok);
                    Err(())
                }
            },
            _ => {
                self.lexer.unget(&tok);
                Err(())
            }
        }
    }

    fn read_variable(&mut self, var: String, range: Range<usize>) -> Result<Node, ()> {
        if self.lexer.skip_symbol(Symbol::Colon)? {
            if let TokenKind::Identifier(name) = self.lexer.read_token()?.kind {
                Ok(Node::new(
                    NodeKind::Variable(var, name.as_str().to_type()),
                    range,
                ))
            } else {
                Err(())
            }
        } else {
            Ok(Node::new(NodeKind::Variable(var, None), range))
        }
    }
}
