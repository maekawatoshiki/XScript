#[derive(Clone, Debug, PartialEq)]
pub enum VMInst {
    PushI(i64),
    PushF(f64),
    PushS(String),

    Pop,

    Call(String),

    Add,
    Sub,
    Mul,
    Div,
    Rem,

    StoreV(usize),
    LoadV(usize),
}
