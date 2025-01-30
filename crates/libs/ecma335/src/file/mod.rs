use super::*;
mod into_stream;

#[derive(Default)]
pub struct File {
    strings: Strings,
    blobs: Blobs,
    tables: Tables,

    TypeRef: HashMap<String, HashMap<String, u32>>,
    AssemblyRef: HashMap<String, u32>,
}

impl File {
    pub fn new(name: &str) -> Self {
        let mut file = Self::default();

        // This assembly.
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

        // This module.
        file.tables.Module.push(Module {
            Name: file.strings.insert(name),
            Mvid: 1,
            ..Default::default()
        });

        // Some parsers will fail to read without an `mscorlib` reference implied by "System" types.
        file.AssemblyRef("System");

        // The parent type of "globals" expected by most parsers.
        file.TypeDef("<Module>", "", TypeAttributes(0), TypeDefOrRef::default());

        file
    }

    pub fn AssemblyRef(&mut self, namespace: &str) -> u32 {
        if let Some(pos) = self.AssemblyRef.get(namespace) {
            return *pos;
        }

        let pos = if namespace == "System" {
            self.tables.AssemblyRef.push_pos(AssemblyRef {
                Name: self.strings.insert("mscorlib"),
                MajorVersion: 4,
                PublicKeyOrToken: self
                    .blobs
                    .insert(&[0xB7, 0x7A, 0x5C, 0x56, 0x19, 0x34, 0xE0, 0x89]),
                ..Default::default()
            })
        } else {
            self.tables.AssemblyRef.push_pos(AssemblyRef {
                Name: self.strings.insert(namespace),
                MajorVersion: 0xFF,
                MinorVersion: 0xFF,
                BuildNumber: 0xFF,
                RevisionNumber: 0xFF,
                Flags: AssemblyFlags::WindowsRuntime,
                ..Default::default()
            })
        };

        self.AssemblyRef.insert(namespace.to_string(), pos);
        pos
    }

    pub fn TypeDef(
        &mut self,
        name: &str,
        namespace: &str,
        flags: TypeAttributes,
        extends: TypeDefOrRef,
    ) -> u32 {
        self.tables.TypeDef.push_pos(TypeDef {
            TypeName: self.strings.insert(name),
            TypeNamespace: self.strings.insert(namespace),
            Flags: flags,
            Extends: extends,
            FieldList: self.tables.Field.len() as u32,
            MethodList: self.tables.MethodDef.len() as u32,
        })
    }

    pub fn TypeRef(&mut self, name: &str, namespace: &str) -> u32 {
        if let Some(key) = self.TypeRef.get(namespace) {
            if let Some(pos) = key.get(name) {
                return *pos;
            }
        }

        let scope = ResolutionScope::AssemblyRef(self.AssemblyRef(namespace));

        let pos = self.tables.TypeRef.push_pos(TypeRef {
            TypeName: self.strings.insert(name),
            TypeNamespace: self.strings.insert(namespace),
            ResolutionScope: scope,
        });

        self.TypeRef
            .entry(namespace.to_string())
            .or_default()
            .insert(name.to_string(), pos);

        pos
    }
}
