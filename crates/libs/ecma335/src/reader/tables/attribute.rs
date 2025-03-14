use super::*;

impl std::fmt::Debug for Attribute<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Attribute").field(&self.ty().parent().name()).finish()
    }
}

impl Attribute<'_> {
    pub fn ty<'a>(&'a self) -> AttributeType<'a> {
        self.decode(1)
    }
}

