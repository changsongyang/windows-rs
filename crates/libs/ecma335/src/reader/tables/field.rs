use super::*;

impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Field").field(&self.name()).finish()
    }
}

impl Field {
    pub fn flags(&self) -> FieldAttributes {
        FieldAttributes(self.usize(0) as u16)
    }

    pub fn name(&self) -> &'static str {
        self.str(1)
    }

    pub fn constant(&self) -> Option<Constant> {
        self.file()
            .equal_range(1, HasConstant::Field(*self).encode())
            .next()
    }
}
