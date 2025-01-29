use super::*;

mod class;
mod cpp_const;
mod cpp_delegate;
mod cpp_enum;
mod cpp_fn;
mod cpp_interface;
mod cpp_struct;
mod delegate;
mod interface;
mod r#enum;
mod r#struct;

pub use class::*;
pub use cpp_const::*;
pub use cpp_delegate::*;
pub use cpp_enum::*;
pub use cpp_fn::*;
pub use cpp_interface::*;
pub use cpp_struct::*;
pub use delegate::*;
pub use interface::*;
pub use r#enum::*;
pub use r#struct::*;

#[derive(Clone, Debug)]
pub enum Type {
    Class(Class),
    CppConst(CppConst),
    CppDelegate(CppDelegate),
    CppEnum(CppEnum),
    CppFn(CppFn),
    CppInterface(CppInterface),
    CppStruct(CppStruct),
    Delegate(Delegate),
    Enum(Enum),
    Interface(Interface),
    Struct(Struct),

    Generic(&'static str),
    PtrMut(Box<Self>, usize),
    PtrConst(Box<Self>, usize),
    ArrayFixed(Box<Self>, usize),
    Array(Box<Self>),
    ArrayRef(Box<Self>),
    ConstRef(Box<Self>),

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
    TypeName(String),
}
