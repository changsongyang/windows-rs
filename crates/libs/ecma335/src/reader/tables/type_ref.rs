use super::*;

impl std::fmt::Debug for TypeRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeRef({}.{})", self.namespace(), self.name())
    }
}

impl TypeRef<'_> {
    pub fn name(&self) -> &str {
        trim_tick(self.str(1))
    }

    pub fn namespace(&self) -> &str {
        self.str(2)
    }
}
