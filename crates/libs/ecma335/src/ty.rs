use super::*;

pub struct TypeName {
    pub namespace: String,
    pub name: String,
    pub generics: Vec<Type>,
}

impl TypeName {
    pub fn new(name: &str, namespace: &str) -> Self {
        Self {
            name: name.to_string(),
            namespace: namespace.to_string(),
            generics: vec![],
        }
    }
}

pub enum Type {
    Void,
    Bool,
    Char,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    F32,
    F64,
    ISize,
    USize,
    String,
    Object,
    Name(TypeName),
}

impl Type {
    pub fn new(name: &str, namespace: &str) -> Self {
        Self::Name(TypeName::new(name, namespace))
    }
}
