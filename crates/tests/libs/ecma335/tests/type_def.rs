use windows_ecma335::*;

#[test]
fn test() {
    let mut file = File::new("test");

    let _type_def = file.TypeDef(
        "Name",
        "Namespace",
        TypeAttributes::WindowsRuntime,
        TypeDefOrRef::default(),
    );

    let _type_ref = file.TypeRef("Guid", "System");

    let bytes = file.into_stream();
    std::fs::write("tests/type_def.winmd", bytes).unwrap();
}
