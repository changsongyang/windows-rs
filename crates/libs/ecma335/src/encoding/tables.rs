use super::*;

#[derive(Default)]
pub struct Assembly {
    pub HashAlgId: u32,
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub RevisionNumber: u16,
    pub Flags: AssemblyFlags,
    pub PublicKey: u32,
    pub Name: u32,
    pub Culture: u32,
}

#[derive(Default)]
pub struct Module {
    pub Generation: u16,
    pub Name: u32,
    pub Mvid: u32,
    pub EncId: u32,
    pub EncBaseId: u32,
}

#[derive(Default)]
pub struct TypeDef {
    pub Flags: u32,
    pub TypeName: u32,
    pub TypeNamespace: u32,
    pub Extends: TypeDefOrRef,
    pub FieldList: u32,
    pub MethodList: u32,
}
