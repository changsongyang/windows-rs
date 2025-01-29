use super::*;
mod into_stream;

#[derive(Default)]
pub struct File {
    // Heaps
    strings: Strings,
    blobs: Blobs,
    
    // Tables
    pub Assembly: Vec<Assembly>,
    pub Module: Vec<Module>,
    pub TypeDef: Vec<TypeDef>,
}

impl File {
    pub fn new(name: &str) -> Self {
        let mut file = Self::default();

        file.TypeDef.push(TypeDef {
            TypeName: file.strings.insert("<Module>"),
            ..Default::default()
        });

        file.Module.push(Module {
            Name: file.strings.insert(name),
            Mvid: 1,
            ..Default::default()
        });

        file.Assembly.push(Assembly {
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
        // TODO: file.insert_scope("System");

        file
    }
}
