macro_rules! tables {
    ($($name:ident)+) => {
        $(
        #[derive(Copy, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
        pub struct $name(u32);
    )*
    };
}

tables! {
    Attribute
    ClassLayout
    Constant
    Field
    GenericParam
    ImplMap
    InterfaceImpl
    MemberRef
    MethodDef
    ModuleRef
    NestedClass
    MethodParam
    TypeDef
    TypeRef
    TypeSpec
}
