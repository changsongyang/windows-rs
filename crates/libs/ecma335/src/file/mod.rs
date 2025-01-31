use super::*;
mod into_stream;

/// Represents an ECMA-335 file in memory so that it can be built incrementally.
#[derive(Default)]
pub struct File {
    strings: Strings,
    blobs: Blobs,
    tables: Tables,

    TypeRef: HashMap<String, HashMap<String, u32>>,
    AssemblyRef: HashMap<String, u32>,
}

impl File {
    /// Creates a minimal ECMA-335 file representation.
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

    /// Adds an `AssemblyRef` row representing the given namespace to the file.
    fn AssemblyRef(&mut self, namespace: &str) -> u32 {
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

    /// Adds a `TypeDef` row to the file.
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

    /// Adds a `TypeRef` row to the file.
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

    /// Adds a `Field` row to the file.
    pub fn Field(&mut self, name: &str, flags: FieldAttributes, ty: &Type) -> u32 {
        let signature = self.FieldSig(ty);

        self.tables.Field.push_pos(Field {
            Name: self.strings.insert(name),
            Flags: flags,
            Signature: signature,
        })
    }

    /// Encodes the `Type` in the buffer. Any required `TypeRef` rows will be added to the file.
    fn Type(&mut self, ty: &Type, buffer: &mut Vec<u8>) {
        match ty {
            Type::Void => buffer.push(ELEMENT_TYPE_VOID),
            Type::Bool => buffer.push(ELEMENT_TYPE_BOOLEAN),
            Type::Char => buffer.push(ELEMENT_TYPE_CHAR),
            Type::I8 => buffer.push(ELEMENT_TYPE_I1),
            Type::U8 => buffer.push(ELEMENT_TYPE_U1),
            Type::I16 => buffer.push(ELEMENT_TYPE_I2),
            Type::U16 => buffer.push(ELEMENT_TYPE_U2),
            Type::I32 => buffer.push(ELEMENT_TYPE_I4),
            Type::U32 => buffer.push(ELEMENT_TYPE_U4),
            Type::I64 => buffer.push(ELEMENT_TYPE_I8),
            Type::U64 => buffer.push(ELEMENT_TYPE_U8),
            Type::F32 => buffer.push(ELEMENT_TYPE_R4),
            Type::F64 => buffer.push(ELEMENT_TYPE_R8),
            Type::ISize => buffer.push(ELEMENT_TYPE_I),
            Type::USize => buffer.push(ELEMENT_TYPE_U),
            Type::String => buffer.push(ELEMENT_TYPE_STRING),
            Type::Object => buffer.push(ELEMENT_TYPE_OBJECT),

            Type::Name(ty) => {
                if !ty.generics.is_empty() {
                    buffer.push(ELEMENT_TYPE_GENERICINST);
                }

                let pos = self.TypeRef(ty.name, ty.namespace);
                buffer.push(ELEMENT_TYPE_VALUETYPE);
                buffer.write_compressed(TypeDefOrRef::TypeRef(pos).encode() as usize);

                if !ty.generics.is_empty() {
                    buffer.write_compressed(ty.generics.len());

                    for ty in &ty.generics {
                        self.Type(ty, buffer);
                    }
                }
            }
        }
    }

    /// Writes the `Type` into a `FileSig` buffer and stores it in the file.
    fn FieldSig(&mut self, ty: &Type) -> u32 {
        let mut buffer = vec![0x6]; // FIELD
        self.Type(ty, &mut buffer);
        self.blobs.insert(&buffer)
    }
}
