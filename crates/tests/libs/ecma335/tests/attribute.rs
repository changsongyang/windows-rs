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

    file.Attribute(
        HasAttribute::TypeDef(def),
        AttributeType::MemberRef(ctor),
        0,
    );

    // TODO: write some attributes

    let bytes = file.into_stream();
    std::fs::write("tests/attribute.winmd", bytes).unwrap();
}
