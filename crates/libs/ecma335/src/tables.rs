use super::*;

#[derive(Default)]
pub struct Tables {
    pub Assembly: Vec<Assembly>,
    pub AssemblyRef: Vec<AssemblyRef>,
    pub Module: Vec<Module>,
    pub TypeDef: Vec<TypeDef>,
}

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
pub struct AssemblyRef {
    pub MajorVersion: u16,
    pub MinorVersion: u16,
    pub BuildNumber: u16,
    pub RevisionNumber: u16,
    pub Flags: u32,
    pub PublicKeyOrToken: u32,
    pub Name: u32,
    pub Culture: u32,
    pub HashValue: u32,
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

impl IntoStream for Tables {
    fn into_stream(self) -> Vec<u8> {
        if [
            self.Assembly.len(),
            self.AssemblyRef.len(),
            self.Module.len(),
            self.TypeDef.len(),
        ]
        .iter()
        .any(|len| *len > u32::MAX as usize)
        {
            panic!("metadata table too large");
        }

        let type_def_or_ref = coded_index_size(&[
            self.TypeDef.len(),
            0, // self.TypeRef.len(),
            0, // self.TypeSpec.len(),
        ]);

        let valid_tables: u64 = (1 << 0) | (1 << 0x02) | (1 << 0x20) | (1 << 0x23);

        // The table stream header...

        let mut buffer = Vec::new();
        buffer.write_u32(0); // Reserved
        buffer.write_u8(2); // MajorVersion
        buffer.write_u8(0); // MinorVersion
        buffer.write_u8(0b111); // HeapSizes
        buffer.write_u8(0); // Reserved
        buffer.write_u64(valid_tables);
        buffer.write_u64(0); // Sorted

        // Followed by the length of each of the valid tables...

        buffer.write_u32(self.Module.len() as u32);
        buffer.write_u32(self.TypeDef.len() as u32);
        buffer.write_u32(self.Assembly.len() as u32);
        buffer.write_u32(self.AssemblyRef.len() as u32);

        // Followed by each table's rows...

        for x in self.Module {
            buffer.write_u16(x.Generation);
            buffer.write_u32(x.Name);
            buffer.write_u32(x.Mvid);
            buffer.write_u32(x.EncId);
            buffer.write_u32(x.EncBaseId);
        }

        for x in &self.TypeDef {
            buffer.write_u32(x.Flags);
            buffer.write_u32(x.TypeName);
            buffer.write_u32(x.TypeNamespace);
            buffer.write_code(x.Extends.encode(), type_def_or_ref);
            buffer.write_index(x.FieldList, 0); // self.Field.len());
            buffer.write_index(x.MethodList, 0); // self.MethodDef.len());
        }

        for x in self.Assembly {
            buffer.write_u32(x.HashAlgId);
            buffer.write_u16(x.MajorVersion);
            buffer.write_u16(x.MinorVersion);
            buffer.write_u16(x.BuildNumber);
            buffer.write_u16(x.RevisionNumber);
            buffer.write_u32(x.Flags.0);
            buffer.write_u32(x.PublicKey);
            buffer.write_u32(x.Name);
            buffer.write_u32(x.Culture);
        }

        for x in self.AssemblyRef {
            buffer.write_u16(x.MajorVersion);
            buffer.write_u16(x.MinorVersion);
            buffer.write_u16(x.BuildNumber);
            buffer.write_u16(x.RevisionNumber);
            buffer.write_u32(x.Flags);
            buffer.write_u32(x.PublicKeyOrToken);
            buffer.write_u32(x.Name);
            buffer.write_u32(x.Culture);
            buffer.write_u32(x.HashValue);
        }

        buffer.into_stream()
    }
}
