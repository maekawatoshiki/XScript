use parser::Parser;
use node::{BinOp, Node, NodeKind};
use vm_base::VMInst;
use typing::Type;

use std::collections::HashMap;

pub struct IdManager {
    counter: usize,
}

impl IdManager {
    pub fn new() -> IdManager {
        IdManager { counter: 0 }
    }

    pub fn get_id(&mut self) -> usize {
        let id = self.counter;
        self.counter += 1;
        id
    }
}

pub type Id = usize;

pub struct Codegen<'a> {
    parser: &'a mut Parser<'a>,
}

impl<'a> Codegen<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Codegen<'a> {
        Codegen { parser: parser }
    }
}

impl<'a> Codegen<'a> {
    pub fn next_inst(&mut self) -> Result<VMInst, ()> {
        let node = self.parser.get_node()?;
        self.gen_inst(node, &HashMap::new())
    }

    pub fn gen_inst(&mut self, node: Node, local_env: &HashMap<String, Id>) -> Result<VMInst, ()> {
        match &node.kind {
            &NodeKind::Int(n) => Ok(VMInst::PushI(n)),
            &NodeKind::Float(f) => Ok(VMInst::PushF(f)),
            &NodeKind::String(ref s) => Ok(VMInst::PushS(s.clone())),
            &NodeKind::Variable(ref name, ref ty) => self.gen_variable(name, ty, local_env),
            // &NodeKind::BinaryOp(BinOp::Add, _, _) => VMInst::Add,
            // &NodeKind::BinaryOp(BinOp::Sub, _, _) => VMInst::Sub,
            // &NodeKind::BinaryOp(BinOp::Mul, _, _) => VMInst::Mul,
            // &NodeKind::BinaryOp(BinOp::Div, _, _) => VMInst::Div,
            // &NodeKind::BinaryOp(BinOp::Rem, _, _) => VMInst::Rem,
            // &NodeKind::BinaryOp(BinOp::Assign, _, _) => self.gen_store(),
            _ => Ok(VMInst::Add),
        }
    }

    pub fn gen_variable(
        &mut self,
        name: &String,
        ty: &Option<Type>,
        local_env: &HashMap<String, Id>,
    ) -> Result<VMInst, ()> {
        Err(())
    }
}

impl<'a> Codegen<'a> {}
