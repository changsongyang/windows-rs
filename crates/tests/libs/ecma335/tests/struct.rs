use windows_ecma335::*;

#[test]
fn test() {
    let mut file = File::new("test");

    let value_type = file.TypeRef("ValueType", "System");

    file.TypeDef(
        "Name",
        "Namespace",
        TypeAttributes::WindowsRuntime,
        TypeDefOrRef::TypeRef(value_type),
    );

    file.Field("SomeGuid", FieldAttributes(0), &Type::new("Guid", "System"));
    file.Field("SomeNum", FieldAttributes(0), &Type::I32);

    let bytes = file.into_stream();
    std::fs::write("tests/struct.winmd", bytes).unwrap();
}
