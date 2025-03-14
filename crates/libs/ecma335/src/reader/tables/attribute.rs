use super::*;

impl std::fmt::Debug for Attribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Attribute").field(&self.name()).finish()
    }
}

impl Attribute {
    pub fn ty(&self) -> AttributeType {
        self.decode(1)
    }

    pub fn name(&self) -> &'static str {
        self.ty().parent().name()
    }
}
