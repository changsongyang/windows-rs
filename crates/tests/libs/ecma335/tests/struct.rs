use windows_ecma335::*;

#[test]
fn test() {
    let mut file = File::new("test");
    let value_type = file.TypeRef("ValueType", "System");

    file.TypeDef(
        "Name",
        "Namespace",
        TypeDefOrRef::TypeRef(value_type),
        TypeAttributes::Public
            | TypeAttributes::SequentialLayout
            | TypeAttributes::Sealed
            | TypeAttributes::WindowsRuntime,
    );

    file.Field(
        "SomeGuid",
        &Type::new("Guid", "System"),
        FieldAttributes::Public,
    );

    file.Field("SomeNum", &Type::I32, FieldAttributes::Public);

    let bytes = file.into_stream();
    std::fs::write("tests/struct.winmd", bytes).unwrap();
}
