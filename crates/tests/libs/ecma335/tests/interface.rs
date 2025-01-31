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

    let flags = MethodAttributes::Public
        | MethodAttributes::HideBySig
        | MethodAttributes::Abstract
        | MethodAttributes::NewSlot
        | MethodAttributes::Virtual;

    let signature = file.MethodDefSig(&[Type::I8, Type::I16], &Type::I32, MethodCallAttributes(0));
    file.MethodDef("One", signature, flags, MethodImplAttributes(0));
    file.Param("i8", 1, ParamAttributes::In);
    file.Param("i16", 2, ParamAttributes::In);

    let signature = file.MethodDefSig(&[], &Type::String, MethodCallAttributes(0));
    file.MethodDef("Two", signature, flags, MethodImplAttributes(0));

    let bytes = file.into_stream();
    std::fs::write("tests/interface.winmd", bytes).unwrap();
}
