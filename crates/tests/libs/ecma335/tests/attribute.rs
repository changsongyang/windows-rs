use windows_ecma335::*;

#[test]
fn test() {
    let mut file = File::new("test");

    file.TypeDef(
        "Name",
        "Namespace",
        TypeDefOrRef::default(),
        TypeAttributes::Public
            | TypeAttributes::Interface
            | TypeAttributes::Abstract
            | TypeAttributes::WindowsRuntime,
    );

    // TODO: write some attributes

    let bytes = file.into_stream();
    std::fs::write("tests/attribute.winmd", bytes).unwrap();
}
