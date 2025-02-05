use super::*;

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
}

impl Value {
    pub fn ty(&self) -> u8 {
        match self {
            Self::Bool(..) => ELEMENT_TYPE_BOOLEAN,
            Self::U8(..) => ELEMENT_TYPE_U1,
            Self::I8(..) => ELEMENT_TYPE_I1,
            Self::U16(..) => ELEMENT_TYPE_U2,
            Self::I16(..) => ELEMENT_TYPE_I2,
            Self::U32(..) => ELEMENT_TYPE_U4,
            Self::I32(..) => ELEMENT_TYPE_I4,
            Self::U64(..) => ELEMENT_TYPE_U8,
            Self::I64(..) => ELEMENT_TYPE_I8,
            Self::F32(..) => ELEMENT_TYPE_R4,
            Self::F64(..) => ELEMENT_TYPE_R8,
        }
    }
}
