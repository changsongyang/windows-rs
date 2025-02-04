pub struct TypeName<'a> {
    pub namespace: &'a str,
    pub name: &'a str,
    pub generics: Vec<Type<'a>>,
}

impl<'a> TypeName<'a> {
    pub fn new(namespace: &'a str, name: &'a str) -> Self {
        Self {
            namespace,
            name,
            generics: vec![],
        }
    }
}

pub enum Type<'a> {
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
    Name(TypeName<'a>),
}

impl<'a> Type<'a> {
    pub fn new(namespace: &'a str, name: &'a str) -> Self {
        Self::Name(TypeName::new(namespace, name))
    }
}
