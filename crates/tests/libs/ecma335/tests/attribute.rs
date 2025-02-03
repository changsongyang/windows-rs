use windows_ecma335::*;

#[test]
fn test() {
    let mut file = File::new("test");

    let def = file.TypeDef(
        "Name",
        "Namespace",
        TypeDefOrRef::default(),
        TypeAttributes::Public
            | TypeAttributes::Interface
            | TypeAttributes::Abstract
            | TypeAttributes::WindowsRuntime,
    );

    let attribute =
        MemberRefParent::TypeRef(file.TypeRef("GuidAttribute", "Windows.Foundation.Metadata"));

    let signature = file.MethodDefSig(
        &[
            Type::U32,
            Type::U16,
            Type::U16,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
            Type::U8,
        ],
        &Type::Void,
        MethodCallAttributes(0),
    );

    let ctor = file.MemberRef(".ctor", signature, attribute);

    let value = file.AttributeValue(
        &[
            Value::U32(1),
            Value::U16(2),
            Value::U16(3),
            Value::U8(4),
            Value::U8(5),
            Value::U8(6),
            Value::U8(7),
            Value::U8(8),
            Value::U8(9),
            Value::U8(10),
            Value::U8(11),
        ],
        &[],
    );

    file.Attribute(
        HasAttribute::TypeDef(def),
        AttributeType::MemberRef(ctor),
        value,
    );

    let bytes = file.into_stream();
    std::fs::write("tests/attribute.winmd", bytes).unwrap();
}
