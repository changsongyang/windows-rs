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

    let signature = file.FieldSig(&Type::new("Guid", "System"));
    file.Field("SomeGuid", signature, FieldAttributes::Public);

    let signature = file.FieldSig(&Type::I32);
    file.Field("SomeNum", signature, FieldAttributes::Public);

    let bytes = file.into_stream();
    std::fs::write("tests/struct.winmd", bytes).unwrap();
}
