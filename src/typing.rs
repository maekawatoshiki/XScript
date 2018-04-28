#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    kind: TypeKind,
}

impl Type {
    pub fn new_int() -> Type {
        Type {
            kind: TypeKind::Int,
        }
    }

    pub fn new_float() -> Type {
        Type {
            kind: TypeKind::Float,
        }
    }

    pub fn new_string() -> Type {
        Type {
            kind: TypeKind::String,
        }
    }
}

pub trait ToType {
    fn to_type(&self) -> Option<Type>;
}

impl<'a> ToType for &'a str {
    fn to_type(&self) -> Option<Type> {
        match self {
            &"int" => Some(Type::new_int()),
            &"float" => Some(Type::new_float()),
            &"string" => Some(Type::new_string()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TypeKind {
    Int,
    Float,
    String,
    // Something...
}
