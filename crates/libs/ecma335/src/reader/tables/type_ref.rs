use super::*;

impl std::fmt::Debug for TypeRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeRef({})", self.type_name())
    }
}

impl TypeRef<'_> {
    pub fn type_name(&self) -> TypeName {
        TypeName(self.namespace(), self.name())
    }

    pub fn name(&self) -> &str {
        trim_tick(self.str(1))
    }

    pub fn namespace(&self) -> &str {
        self.str(2)
    }
}
