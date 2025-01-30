use super::*;
mod into_stream;

#[derive(Default)]
pub struct File {
    strings: Strings,
    blobs: Blobs,
    tables: Tables,
}

impl File {
    pub fn new(name: &str) -> Self {
        let mut file = Self::default();

        file.tables.TypeDef.push(TypeDef {
            TypeName: file.strings.insert("<Module>"),
            ..Default::default()
        });

        file.tables.Module.push(Module {
            Name: file.strings.insert(name),
            Mvid: 1,
            ..Default::default()
        });

        file.tables.Assembly.push(Assembly {
            Name: file.strings.insert(name),
            HashAlgId: 0x00008004,
            MajorVersion: 0xFF,
            MinorVersion: 0xFF,
            BuildNumber: 0xFF,
            RevisionNumber: 0xFF,
            Flags: AssemblyFlags::WindowsRuntime,
            ..Default::default()
        });

        // Some parsers will fail to read without an `mscorlib` reference.
        file.tables.AssemblyRef.push(AssemblyRef {
            Name: file.strings.insert("mscorlib"),
            MajorVersion: 4,
            PublicKeyOrToken: file
                .blobs
                .insert(&[0xB7, 0x7A, 0x5C, 0x56, 0x19, 0x34, 0xE0, 0x89]),
            ..Default::default()
        });

        file
    }
}
