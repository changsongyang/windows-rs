macro_rules! code {
    ($name:ident($size:literal) $(($table:ident, $code:literal))+) => {
        #[derive(Clone, Copy)]
        pub enum $name {
            $($table(u32),)*
        }
        impl $name {
            pub fn encode(&self) -> u32 {
                match self {
                    $(Self::$table(row) => (row.overflowing_add(1).0) << $size | $code,)*
                }
            }
        }
    };
}

code! { TypeDefOrRef(2)
    (TypeDef, 0)
    (TypeRef, 1)
    (TypeSpec, 2)
}

impl Default for TypeDefOrRef {
    fn default() -> Self {
        // TODO: why is this "none"?
        TypeDefOrRef::TypeDef(u32::MAX)
    }
}

code! { ResolutionScope(2)
    (Module, 0)
    (ModuleRef, 1)
    (AssemblyRef, 2)
    (TypeRef, 3)
}
