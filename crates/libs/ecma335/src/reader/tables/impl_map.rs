use super::*;

impl std::fmt::Debug for ImplMap<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ImplMap").field(&self.import_name()).finish()
    }
}

impl<'a> ImplMap<'a> {
    pub fn flags(&self) -> PInvokeAttributes {
        PInvokeAttributes(self.usize(0))
    }

    pub fn scope(&'a self) -> ModuleRef<'a> {
        ModuleRef(self.row(3))
    }

    pub fn import_name(&'a self) -> &'a str {
        self.str(2)
    }
}
