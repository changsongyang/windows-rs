use super::*;

impl std::fmt::Debug for TypeDef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeDef({})", self.type_name())
    }
}

impl TypeDef<'_> {
    pub fn flags(&self) -> TypeAttributes {
        TypeAttributes(self.usize(0) as u32)
    }

    pub fn type_name<'a>(&'a self) -> TypeName<'a> {
        TypeName(self.namespace(), self.name())
    }

    pub fn name<'a>(&'a self) -> &'a str {
        trim_tick(self.str(1))
    }

    pub fn raw_name<'a>(&'a self) -> &'a str {
        self.str(1)
    }

    pub fn namespace<'a>(&'a self) -> &'a str {
        self.str(2)
    }

    pub fn extends(& self) -> Option<TypeDefOrRef> {
        let extends = self.usize(3);

        if extends == 0 {
            return None;
        }

        Some(TypeDefOrRef::decode(self.file(), extends))
    }

    pub fn methods(&self) -> RowIterator<MethodDef> {
        self.list(5)
    }

    pub fn fields(&self) -> RowIterator<Field> {
        self.list(4)
    }

    pub fn generic_params(&self) -> RowIterator<GenericParam> {
        self.file()
            .equal_range(2, TypeOrMethodDef::TypeDef(*self).encode())
    }

    pub fn interface_impls(&self) -> RowIterator<InterfaceImpl> {
        self.file().equal_range(0, self.index() + 1)
    }

    pub fn class_layout(&self) -> Option<ClassLayout> {
        self.file().equal_range(2, self.index() + 1).next()
    }

    pub fn nested(&self) -> Option<NestedClass> {
        self.file().equal_range(0, self.index() + 1).next()
    }

    pub fn is_async(&self) -> bool {
        matches!(
            TypeName(self.namespace(), self.name()),
            TypeName::IAsyncAction
                | TypeName::IAsyncActionWithProgress
                | TypeName::IAsyncOperation
                | TypeName::IAsyncOperationWithProgress
        )
    }
}
