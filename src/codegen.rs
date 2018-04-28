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
    pub parser: &'a mut Parser<'a>,
    pub id_manager: IdManager,
    pub vm_insts: Vec<VMInst>,
}

impl<'a> Codegen<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Codegen<'a> {
        Codegen {
            parser: parser,
            id_manager: IdManager::new(),
            vm_insts: Vec::new(),
        }
    }
}

impl<'a> Codegen<'a> {
    pub fn gen(&mut self) {
        let mut local_env = HashMap::new();
        while let Ok(node) = self.parser.get_node() {
            self.gen_inst(&node, &mut local_env).unwrap();
        }
        self.vm_insts.insert(0, VMInst::Entry(local_env.len()));
        self.vm_insts.push(VMInst::Ret);
    }

    pub fn gen_inst(&mut self, node: &Node, local_env: &mut HashMap<String, Id>) -> Result<(), ()> {
        match node.kind {
            NodeKind::Int(n) => self.vm_insts.push(VMInst::PushI(n)),
            NodeKind::Float(f) => self.vm_insts.push(VMInst::PushF(f)),
            NodeKind::String(ref s) => self.vm_insts.push(VMInst::PushS(s.clone())),
            NodeKind::Variable(ref name, ref ty) => self.gen_variable(name, ty, local_env)?,
            NodeKind::BinaryOp(ref lhs, ref rhs, BinOp::Assign) => {
                self.gen_store(&*lhs, &*rhs, local_env)?
            }
            NodeKind::BinaryOp(ref lhs, ref rhs, ref op) => {
                self.gen_binop(&*lhs, &*rhs, &*op, local_env)?
            }
            _ => {}
        };
        Ok(())
    }

    pub fn gen_binop(
        &mut self,
        lhs: &Node,
        rhs: &Node,
        op: &BinOp,
        local_env: &mut HashMap<String, Id>,
    ) -> Result<(), ()> {
        self.gen_inst(lhs, local_env)?;
        self.gen_inst(rhs, local_env)?;
        self.vm_insts.push(match op {
            &BinOp::Add => VMInst::Add,
            &BinOp::Sub => VMInst::Sub,
            &BinOp::Mul => VMInst::Mul,
            &BinOp::Div => VMInst::Div,
            &BinOp::Rem => VMInst::Rem,
            _ => unimplemented!(),
        });
        Ok(())
    }

    pub fn gen_variable(
        &mut self,
        name: &String,
        ty: &Option<Type>,
        local_env: &mut HashMap<String, Id>,
    ) -> Result<(), ()> {
        if let Some(id) = local_env.get(name) {
            self.vm_insts.push(VMInst::LoadV(*id))
        } else {
            panic!("TODO: implement err handler");
        }
        Ok(())
    }

    pub fn gen_store(
        &mut self,
        lhs: &Node,
        rhs: &Node,
        local_env: &mut HashMap<String, Id>,
    ) -> Result<(), ()> {
        let var_id = match lhs.kind {
            NodeKind::Variable(ref name, _) => *local_env
                .entry(name.clone())
                .or_insert_with(|| self.id_manager.get_id()),
            _ => unimplemented!(),
        };
        self.gen_inst(rhs, local_env)?;
        self.vm_insts.push(VMInst::StoreV(var_id));
        Ok(())
    }
}

impl<'a> Codegen<'a> {}
