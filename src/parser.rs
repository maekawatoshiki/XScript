use node::Node;
use token;
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

impl<'a> Parser<'a> {
    // pub fn get_node(&mut self) -> Result<Node, ()> {
    //     self.read_toplevel()
    // }
    //
    // pub fn read_toplevel(&mut self) -> Result<Node, ()> {
    //     let tok = self.lexer.read_token()?;
    //     match tok.kind {
    //         token::TokenKind::Identifier(name) if name == "if" => unimplemented!(),
    //         _ => self.read_expr(),
    //     }
    // }
}
