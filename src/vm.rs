use vm_base::VMInst;

use ansi_term::{Colour, Style};

#[derive(Clone)]
pub struct VM {
    pub stack: [i64; 1024],
    pub bp_stack: Vec<usize>,
    pub sp: usize,
    pub bp: usize,
}

impl VM {
    pub fn new() -> VM {
        VM {
            stack: [0; 1024],
            bp_stack: Vec::new(),
            sp: 0,
            bp: 0,
        }
    }
}

impl VM {
    pub fn run(&mut self, insts: Vec<VMInst>) {
        for inst in insts {
            self.run_inst(inst.clone());
            for i in 0..8 {
                print!(
                    "{}{}{} ",
                    if i == self.sp {
                        Colour::Red.paint("[")
                    } else if i == self.bp {
                        Colour::Green.paint("[")
                    } else {
                        Style::new().bold().paint("[")
                    },
                    self.stack[i],
                    if i == self.bp {
                        Colour::Green.paint("]")
                    } else if i == self.sp {
                        Colour::Red.paint("]")
                    } else {
                        Style::new().bold().paint("]")
                    },
                );
            }
            println!("\t\t:{:?}", inst);
        }
    }

    pub fn run_inst(&mut self, inst: VMInst) {
        match inst {
            VMInst::Entry(n) => {
                self.bp_stack.push(self.bp);
                self.sp += n;
                self.bp = self.sp;
            }
            VMInst::StoreV(n) => self.stack[self.bp - 1 - n] = self.stack[self.sp],
            VMInst::PushI(n) => {
                self.sp += 1;
                self.stack[self.sp] = n;
            }
            VMInst::Add => {
                let a = self.stack[self.sp];
                let b = self.stack[self.sp - 1];
                self.sp -= 1;
                self.stack[self.sp] = a + b;
            }
            VMInst::Ret => {
                self.bp = self.bp_stack.pop().unwrap();
                self.sp = self.bp
            }
            _ => {}
        }
    }
}
