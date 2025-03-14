use super::*;

#[derive(Debug)]
pub enum Value {
    Bool(bool),
    U8(u8),
    I8(i8),
    U16(u16),
    I16(i16),
    U32(u32),
    I32(i32),
    U64(u64),
    I64(i64),
    F32(f32),
    F64(f64),
    Str(&'static str),
    TypeName(&'static str),
}

impl Value {
    pub fn ty(&self) -> ElementType {
        match self {
            Self::Bool(..) => ElementType::Bool,
            Self::U8(..) => ElementType::U8,
            Self::I8(..) => ElementType::I8,
            Self::U16(..) => ElementType::U16,
            Self::I16(..) => ElementType::I16,
            Self::U32(..) => ElementType::U32,
            Self::I32(..) => ElementType::I32,
            Self::U64(..) => ElementType::U64,
            Self::I64(..) => ElementType::I64,
            Self::F32(..) => ElementType::F32,
            Self::F64(..) => ElementType::F64,
            Self::Str(..) => ElementType::String,
            rest => panic!("{rest:?}"),
        }
    }
}
