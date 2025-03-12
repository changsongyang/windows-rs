use windows_bindgen as r;
use windows_ecma335 as w;

fn main() {
    let time = std::time::Instant::now();

    let input = r::Reader::new(vec![r::File::new(
        std::fs::read("crates/libs/bindgen/default/Windows.winmd").unwrap(),
    )
    .unwrap()]);

    let mut output = w::File::new("test");

    for ty in input.values().flat_map(|values| values.values()).flatten() {
        write_type(&mut output, ty);
    }

    let bytes = output.into_stream();
    std::fs::write("copy.winmd", bytes).unwrap();

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}

fn write_type(output: &mut w::File, ty: &r::Type) {
    match ty {
        r::Type::Enum(ty) => write_def(output, ty.def, true),
        r::Type::Struct(ty) => write_def(output, ty.def, true),
        r::Type::Delegate(ty) => write_def(output, ty.def, true),
        r::Type::Interface(ty) => write_def(output, ty.def, true),
        r::Type::Class(ty) => write_def(output, ty.def, false),
        _ => {}
    }
}

fn write_attributes<R: r::HasAttributes>(output: &mut w::File, parent: w::HasAttribute, row: R) {
    for attribute in row.attributes() {
        let ty = attribute.ty().parent();
        let attribute_ref = w::MemberRefParent::TypeRef(output.TypeRef(ty.namespace(), ty.name()));
        let args = attribute.args();

        let types: Vec<w::Type> = args
            .iter()
            .filter_map(|(name, value)| {
                if name.is_empty() {
                    Some(convert_value(value).ty())
                } else {
                    None
                }
            })
            .collect();

        let signature = output.MethodDefSig(&types, &w::Type::Void, w::MethodCallAttributes(0));
        let ctor = output.MemberRef(".ctor", signature, attribute_ref);

        let fixed: Vec<w::Value> = args
            .iter()
            .filter_map(|(name, value)| {
                if name.is_empty() {
                    Some(convert_value(value))
                } else {
                    None
                }
            })
            .collect();

        let named: Vec<(&str, w::Value)> = args
            .iter()
            .filter_map(|(name, value)| {
                if !name.is_empty() {
                    Some((*name, convert_value(value)))
                } else {
                    None
                }
            })
            .collect();

        let value = output.AttributeValue(&fixed, &named);
        output.Attribute(parent, w::AttributeType::MemberRef(ctor), value);
    }
}

fn write_def(output: &mut w::File, def: r::TypeDef, include_methods: bool) {
    let flags = w::TypeAttributes(def.flags().0);

    let extends = if let Some(extends) = def.extends() {
        w::TypeDefOrRef::TypeRef(output.TypeRef(extends.namespace(), extends.name()))
    } else {
        w::TypeDefOrRef::default()
    };

    let type_def = output.TypeDef(def.namespace(), def.name(), extends, flags);

    for field in def.fields() {
        let flags = w::FieldAttributes(field.flags().0);
        let signature = output.FieldSig(&convert_type(&field.ty(None)));

        let parent = output.Field(field.name(), signature, flags);

        if let Some(constant) = field.constant() {
            let value = convert_value(&constant.value());
            let ty = value.ty().code();
            let value = output.ConstantValue(&value);

            output.Constant(w::HasConstant::Field(parent), ty, value);
        }
    }

    // TODO: may need to call write_attributes for methods/parameters/fields for Win32 metadata
    write_attributes(output, w::HasAttribute::TypeDef(type_def), def);

    let generics = def.generics();

    for interface in def.interface_impls() {
        let interface = convert_type(&interface.ty(&generics));

        let interface = if let w::Type::Name(tn) = interface {
            if tn.generics.is_empty() {
                w::TypeDefOrRef::TypeRef(output.TypeRef(tn.namespace, tn.name))
            } else {
                w::TypeDefOrRef::TypeSpec(output.TypeSpec(&tn))
            }
        } else {
            panic!();
        };

        output.InterfaceImpl(type_def, interface);
    }

    for generic in def.generic_params() {
        output.GenericParam(
            generic.name(),
            w::TypeOrMethodDef::TypeDef(type_def),
            generic.sequence(),
            generic.flags(),
        );
    }

    // Methods on classes is a huge overhead on .winmd size but adds no value as all of this information
    // is redundantly stored elsewhere.
    if include_methods {
        for method in def.methods() {
            let signature = method.signature("", &generics);
            let types: Vec<w::Type> = signature
                .params
                .iter()
                .map(|param| convert_type(&param.ty))
                .collect();
            let signature_blob = output.MethodDefSig(
                &types,
                &convert_type(&signature.return_type),
                w::MethodCallAttributes(signature.call_flags.0),
            );
            let flags = w::MethodAttributes(method.flags().0);
            let impl_flags = w::MethodImplAttributes(method.impl_flags().0);

            output.MethodDef(method.name(), signature_blob, flags, impl_flags);

            for param in signature.params {
                output.Param(
                    param.def.name(),
                    param.def.sequence(),
                    w::ParamAttributes(param.def.flags().0),
                );
            }
        }
    }
}

fn convert_type(input: &r::Type) -> w::Type<'static> {
    match input {
        r::Type::Void => w::Type::Void,
        r::Type::Bool => w::Type::Bool,
        r::Type::Char => w::Type::Char,
        r::Type::I8 => w::Type::I8,
        r::Type::U8 => w::Type::U8,
        r::Type::I16 => w::Type::I16,
        r::Type::U16 => w::Type::U16,
        r::Type::I32 => w::Type::I32,
        r::Type::U32 => w::Type::U32,
        r::Type::I64 => w::Type::I64,
        r::Type::U64 => w::Type::U64,
        r::Type::F32 => w::Type::F32,
        r::Type::F64 => w::Type::F64,
        r::Type::ISize => w::Type::ISize,
        r::Type::USize => w::Type::USize,
        r::Type::String => w::Type::String,
        r::Type::Object => w::Type::Object,
        r::Type::GUID => w::Type::new("System", "Guid"),
        // TODO: Type::HRESULT is ambiguous... since it can refer to either the WinRT or Win32 HRESULT
        r::Type::HRESULT => w::Type::new("Windows.Foundation", "HResult"),
        r::Type::Array(ty) => w::Type::Array(Box::new(convert_type(ty))),
        r::Type::ArrayRef(ty) => w::Type::ArrayRef(Box::new(convert_type(ty))),
        r::Type::ConstRef(ty) => w::Type::ConstRef(Box::new(convert_type(ty))),
        r::Type::Enum(ty) => w::Type::new(ty.def.namespace(), ty.def.name()),
        r::Type::Struct(ty) => w::Type::new(ty.def.namespace(), ty.def.name()),
        r::Type::Class(ty) => w::Type::new(ty.def.namespace(), ty.def.name()),
        r::Type::Delegate(ty) => w::Type::new(ty.def.namespace(), ty.def.name()),
        r::Type::Interface(ty) => w::Type::Name(w::TypeName {
            namespace: ty.def.namespace(),
            name: ty.def.name(),
            generics: ty.generics.iter().map(|ty| convert_type(ty)).collect(),
        }),
        r::Type::Generic(name) => w::Type::Generic(name.sequence() as usize),
        rest => panic!("{rest:?}"),
    }
}

fn convert_value(value: &r::Value) -> w::Value {
    match value {
        r::Value::Bool(value) => w::Value::Bool(*value),
        r::Value::U8(value) => w::Value::U8(*value),
        r::Value::I8(value) => w::Value::I8(*value),
        r::Value::U16(value) => w::Value::U16(*value),
        r::Value::I16(value) => w::Value::I16(*value),
        r::Value::U32(value) => w::Value::U32(*value),
        r::Value::I32(value) => w::Value::I32(*value),
        r::Value::U64(value) => w::Value::U64(*value),
        r::Value::I64(value) => w::Value::I64(*value),
        r::Value::F32(value) => w::Value::F32(*value),
        r::Value::F64(value) => w::Value::F64(*value),
        r::Value::Str(value) => w::Value::Str(value),
        r::Value::TypeName(tn) => w::Value::TypeName(w::TypeName::new(tn.namespace(), tn.name())),
        rest => panic!("{rest:?}"),
    }
}
