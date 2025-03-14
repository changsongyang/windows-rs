use super::*;

impl std::fmt::Debug for MemberRef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MemberRef").field(&self.0).finish()
    }
}

impl<'a> MemberRef<'a> {
    pub fn parent(&'a self) -> MemberRefParent<'a> {
        self.decode(0)
    }
}
