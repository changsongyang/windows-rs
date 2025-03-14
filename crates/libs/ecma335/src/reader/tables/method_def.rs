use super::*;

impl std::fmt::Debug for MethodDef<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("MethodDef").field(&self.name()).finish()
    }
}

impl<'a> MethodDef<'a> {
    pub fn impl_flags(&self) -> MethodImplAttributes {
        MethodImplAttributes(self.usize(1) as u16)
    }

    pub fn flags(&self) -> MethodAttributes {
        MethodAttributes(self.usize(2) as u16)
    }

    pub fn name(&'a self) -> &'a str {
        self.str(3)
    }

    // pub fn import_name(&'a self) -> Option<&'a str> {
    //     self.impl_map().and_then(|map| {
    //         let import_name = map.import_name();
    //         if self.name() != import_name {
    //             Some(import_name)
    //         } else {
    //             None
    //         }
    //     })
    // }

    pub fn params(&self) -> RowIterator<MethodParam> {
        self.list(5)
    }

    pub fn parent(&'a self) -> MemberRefParent<'a> {
        MemberRefParent::TypeDef(self.file().parent(5, *self))
    }

    pub fn impl_map(&self) -> Option<ImplMap> {
        self.file()
            .equal_range(1, MemberForwarded::MethodDef(*self).encode())
            .next()
    }

    // pub fn module_name(&'a self) -> &'a str {
    //     self.impl_map().map_or("", |map| map.scope().name())
    // }

    pub fn calling_convention(&self) -> &str {
        self.impl_map().map_or("", |map| {
            let flags = map.flags();

            if flags.contains(PInvokeAttributes::CallConvPlatformapi) {
                "system"
            } else if flags.contains(PInvokeAttributes::CallConvCdecl) {
                "cdecl"
            } else {
                ""
            }
        })
    }
}
