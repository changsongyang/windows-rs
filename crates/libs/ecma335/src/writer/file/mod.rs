use super::*;
use std::collections::*;
mod into_stream;

mod records;

mod blobs;
use blobs::*;

mod strings;
use strings::*;

mod helpers;
use helpers::*;

/// Represents an ECMA-335 file in memory so that it can be built incrementally.
#[derive(Default)]
pub struct File {
    strings: Strings,
    blobs: Blobs,
    records: records::Records,

    // Indexes for fast lookup of preexisting rows.
    TypeRef: HashMap<String, HashMap<String, u32>>,
    AssemblyRef: HashMap<String, u32>,

    // Staging for sorted rows before these records can be written. BTreeMap is used rather than HashMap to allow reproducible builds.
    Constant: BTreeMap<HasConstant, records::Constant>,
    Attribute: BTreeMap<HasAttribute, Vec<records::Attribute>>,
    GenericParam: BTreeMap<TypeOrMethodDef, Vec<records::GenericParam>>,
}

impl File {
    /// Creates a minimal ECMA-335 file representation.
    pub fn new(name: &str) -> Self {
        let mut file = Self::default();

        // This assembly.
        file.records.Assembly.push(records::Assembly {
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
        file.records.Module.push(records::Module {
            Name: file.strings.insert(name),
            Mvid: 1,
            ..Default::default()
        });

        // Some parsers will fail to read without an `mscorlib` reference implied by "System" types.
        file.AssemblyRef("System");

        // The parent type of "globals" expected by most parsers.
        file.TypeDef("", "<Module>", TypeDefOrRef::zeroed(), TypeAttributes(0));

        file
    }

    /// Adds an `AssemblyRef` row representing the given namespace to the file, returning the row offset.
    fn AssemblyRef(&mut self, namespace: &str) -> AssemblyRef {
        // This generates a synthetic `AssemblyRef` for every root namespace, but the alternative requires a
        // lot more contextual information which we can hopefully avoid for now.
        let namespace = namespace
            .split_once('.')
            .map_or(namespace, |(prefix, _)| prefix);

        if let Some(pos) = self.AssemblyRef.get(namespace) {
            return AssemblyRef(*pos);
        }

        let pos = if namespace == "System" {
            self.records.AssemblyRef.push_pos(records::AssemblyRef {
                Name: self.strings.insert("mscorlib"),
                MajorVersion: 4,
                PublicKeyOrToken: self
                    .blobs
                    .insert(&[0xB7, 0x7A, 0x5C, 0x56, 0x19, 0x34, 0xE0, 0x89]),
                ..Default::default()
            })
        } else {
            self.records.AssemblyRef.push_pos(records::AssemblyRef {
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
        AssemblyRef(pos)
    }

    /// Adds a `TypeDef` row to the file, returning the row offset.
    pub fn TypeDef(
        &mut self,
        namespace: &str,
        name: &str,
        extends: TypeDefOrRef,
        flags: TypeAttributes,
    ) -> TypeDef {
        TypeDef(self.records.TypeDef.push_pos(records::TypeDef {
            TypeName: self.strings.insert(name),
            TypeNamespace: self.strings.insert(namespace),
            Flags: flags,
            Extends: extends,
            FieldList: self.records.Field.len() as u32,
            MethodList: self.records.MethodDef.len() as u32,
        }))
    }

    /// Adds a `TypeRef` row to the file, returning the row offset.
    pub fn TypeRef(&mut self, namespace: &str, name: &str) -> TypeRef {
        if let Some(key) = self.TypeRef.get(namespace) {
            if let Some(pos) = key.get(name) {
                return TypeRef(*pos);
            }
        }

        // The type may be local to the module but that requires more contextual information.
        let scope = ResolutionScope::AssemblyRef(self.AssemblyRef(namespace));

        let pos = self.records.TypeRef.push_pos(records::TypeRef {
            TypeName: self.strings.insert(name),
            TypeNamespace: self.strings.insert(namespace),
            ResolutionScope: scope,
        });

        self.TypeRef
            .entry(namespace.to_string())
            .or_default()
            .insert(name.to_string(), pos);

        TypeRef(pos)
    }

    pub fn TypeSpec(&mut self, tn: &TypeName) -> TypeSpec {
        debug_assert!(!tn.generics.is_empty());

        let type_ref = self.TypeRef(tn.namespace, tn.name);

        let mut buffer = vec![];
        buffer.push(ELEMENT_TYPE_GENERICINST);
        buffer.push(ELEMENT_TYPE_CLASS);
        buffer.write_compressed(TypeDefOrRef::TypeRef(type_ref).encode() as usize);
        buffer.write_compressed(tn.generics.len());

        for ty in &tn.generics {
            self.Type(ty, &mut buffer);
        }

        TypeSpec(self.records.TypeSpec.push_pos(records::TypeSpec {
            Signature: self.blobs.insert(&buffer),
        }))
    }

    /// Adds a `Field` row to the file, returning the row offset.
    pub fn Field(&mut self, name: &str, signature: FieldSig, flags: FieldAttributes) -> Field {
        Field(self.records.Field.push_pos(records::Field {
            Name: self.strings.insert(name),
            Flags: flags,
            Signature: signature.0,
        }))
    }

    /// Adds a `MethodDef` row to the file, returning the row offset.
    pub fn MethodDef(
        &mut self,
        name: &str,
        signature: MethodDefSig,
        flags: MethodAttributes,
        impl_flags: MethodImplAttributes,
    ) -> MethodDef {
        MethodDef(self.records.MethodDef.push_pos(records::MethodDef {
            RVA: 0,
            ImplFlags: impl_flags,
            Flags: flags,
            Name: self.strings.insert(name),
            Signature: signature.0,
            ParamList: self.records.Param.len() as u32,
        }))
    }

    pub fn MemberRef(
        &mut self,
        name: &str,
        signature: MethodDefSig,
        parent: MemberRefParent,
    ) -> MemberRef {
        MemberRef(self.records.MemberRef.push_pos(records::MemberRef {
            Name: self.strings.insert(name),
            Signature: signature.0,
            Parent: parent,
        }))
    }

    /// Adds a `Param` row to the file, returning the row offset.
    pub fn Param(&mut self, name: &str, sequence: u16, flags: ParamAttributes) -> Param {
        Param(self.records.Param.push_pos(records::Param {
            Flags: flags,
            Sequence: sequence,
            Name: self.strings.insert(name),
        }))
    }

    /// Adds an `Attribute` row to the file. This is a sorted table so the row offset is not yet available.
    pub fn Attribute(&mut self, parent: HasAttribute, ty: AttributeType, value: AttributeValue) {
        self.Attribute
            .entry(parent)
            .or_default()
            .push(records::Attribute {
                Parent: parent,
                Type: ty,
                Value: value.0,
            });
    }

    pub fn Constant(&mut self, parent: HasConstant, ty: u8, value: ConstantValue) {
        self.Constant.insert(
            parent,
            records::Constant {
                Parent: parent,
                Type: ty,
                Value: value.0,
            },
        );
    }

    pub fn GenericParam(&mut self, name: &str, owner: TypeOrMethodDef, number: u16, flags: u16) {
        self.GenericParam
            .entry(owner)
            .or_default()
            .push(records::GenericParam {
                Name: self.strings.insert(name),
                Number: number,
                Owner: owner,
                Flags: flags,
            });
    }

    pub fn InterfaceImpl(&mut self, type_def: TypeDef, interface: TypeDefOrRef) -> InterfaceImpl {
        InterfaceImpl(self.records.InterfaceImpl.push_pos(records::InterfaceImpl {
            Class: type_def.0,
            Interface: interface,
        }))
    }

    /// Encodes the `Type` in the buffer. Any required `TypeRef` rows will be added to the file, returning the blob offset.
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

            Type::Array(ty) => {
                buffer.push(ELEMENT_TYPE_SZARRAY);
                self.Type(ty, buffer);
            }

            Type::ArrayRef(ty) => {
                buffer.push(ELEMENT_TYPE_BYREF);
                buffer.push(ELEMENT_TYPE_SZARRAY);
                self.Type(ty, buffer);
            }

            Type::ConstRef(ty) => {
                buffer.write_compressed(ELEMENT_TYPE_CMOD_REQD as usize);
                let pos = self.TypeRef("System.Runtime.CompilerServices", "IsConst");
                buffer.write_compressed(TypeDefOrRef::TypeRef(pos).encode() as usize);
                self.Type(ty, buffer);
            }

            Type::PtrMut(ty, pointers) => {
                for _ in 0..*pointers {
                    buffer.write_compressed(ELEMENT_TYPE_PTR as usize);
                }

                self.Type(ty, buffer);
            }

            Type::PtrConst(ty, pointers) => {
                buffer.write_compressed(ELEMENT_TYPE_CMOD_REQD as usize);
                let pos = self.TypeRef("System.Runtime.CompilerServices", "IsConst");
                buffer.write_compressed(TypeDefOrRef::TypeRef(pos).encode() as usize);

                for _ in 0..*pointers {
                    buffer.write_compressed(ELEMENT_TYPE_PTR as usize);
                }

                self.Type(ty, buffer);
            }

            Type::ArrayFixed(ty, len) => {
                buffer.push(ELEMENT_TYPE_ARRAY);
                self.Type(ty, buffer);
                buffer.write_compressed(1);
                buffer.write_compressed(1);
                buffer.write_compressed(*len);
            }

            Type::Name(ty) => {
                if !ty.generics.is_empty() {
                    buffer.push(ELEMENT_TYPE_GENERICINST);
                }

                let pos = self.TypeRef(ty.namespace, ty.name);
                // Technically this should be ELEMENT_TYPE_CLASS if the type is not a value type but that requires more contextual information.
                // TODO: we could replace Type::Name with Type::Value and Type::Class to provide this context if needed.
                buffer.push(ELEMENT_TYPE_VALUETYPE);
                buffer.write_compressed(TypeDefOrRef::TypeRef(pos).encode() as usize);

                if !ty.generics.is_empty() {
                    buffer.write_compressed(ty.generics.len());

                    for ty in &ty.generics {
                        self.Type(ty, buffer);
                    }
                }
            }

            Type::Generic(number) => {
                buffer.push(ELEMENT_TYPE_VAR);
                buffer.write_compressed(*number);
            }
            Type::Type => self.Type(&Type::new("System", "Type"), buffer),
        }
    }

    /// Writes the `Type` into a `FileSig` buffer and stores it in the file, returning the blob offset.
    pub fn FieldSig(&mut self, ty: &Type) -> FieldSig {
        let mut buffer = vec![0x6]; // FIELD
        self.Type(ty, &mut buffer);
        FieldSig(self.blobs.insert(&buffer))
    }

    /// Writes the method signature into a `MethodDefSig` buffer and stores it in the file, returning the blob offset.
    pub fn MethodDefSig(
        &mut self,
        params: &[Type],
        return_type: &Type,
        flags: MethodCallAttributes,
    ) -> MethodDefSig {
        let mut buffer = vec![flags.0];
        buffer.write_compressed(params.len());
        self.Type(return_type, &mut buffer);

        for ty in params {
            self.Type(ty, &mut buffer);
        }

        MethodDefSig(self.blobs.insert(&buffer))
    }

    pub fn ConstantValue(&mut self, value: &Value) -> ConstantValue {
        let mut buffer = vec![];
        buffer.write_value(value);
        ConstantValue(self.blobs.insert(&buffer))
    }

    pub fn AttributeValue(&mut self, fixed: &[Value], named: &[(&str, Value)]) -> AttributeValue {
        let mut buffer = vec![];
        buffer.write_u16(1); // prolog

        for value in fixed {
            buffer.write_value(value);
        }

        buffer.write_u16(named.len().try_into().unwrap());

        for (name, value) in named {
            buffer.push(0x53); // field=0x53 property=0x54
            buffer.push(value.ty());
            buffer.write_compressed(name.len());
            buffer.extend_from_slice(name.as_bytes());
            buffer.write_value(value);
        }

        AttributeValue(self.blobs.insert(&buffer))
    }
}
