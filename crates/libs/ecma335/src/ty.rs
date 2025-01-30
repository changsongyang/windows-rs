use super::*;

pub struct TypeName {
    pub namespace: String,
    pub name: String,
    pub generics: Vec<Type>,
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
    Named(TypeName),
}

impl Type {
    pub fn write(&self, _file: &mut File, buffer: &mut Vec<u8>) {
        match self {
            Self::Void => buffer.push(ELEMENT_TYPE_VOID),
            Self::Bool => buffer.push(ELEMENT_TYPE_BOOLEAN),
            Self::Char => buffer.push(ELEMENT_TYPE_CHAR),
            Self::I8 => buffer.push(ELEMENT_TYPE_I1),
            Self::U8 => buffer.push(ELEMENT_TYPE_U1),
            Self::I16 => buffer.push(ELEMENT_TYPE_I2),
            Self::U16 => buffer.push(ELEMENT_TYPE_U2),
            Self::I32 => buffer.push(ELEMENT_TYPE_I4),
            Self::U32 => buffer.push(ELEMENT_TYPE_U4),
            Self::I64 => buffer.push(ELEMENT_TYPE_I8),
            Self::U64 => buffer.push(ELEMENT_TYPE_U8),
            Self::F32 => buffer.push(ELEMENT_TYPE_R4),
            Self::F64 => buffer.push(ELEMENT_TYPE_R8),
            Self::ISize => buffer.push(ELEMENT_TYPE_I),
            Self::USize => buffer.push(ELEMENT_TYPE_U),
            Self::String => buffer.push(ELEMENT_TYPE_STRING),
            Self::Object => buffer.push(ELEMENT_TYPE_OBJECT),
            Self::Named(_name) => {}
        }
    }
}
