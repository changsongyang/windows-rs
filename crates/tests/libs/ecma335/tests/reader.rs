use windows_ecma335::reader::*;

#[test]
fn test() {
    let file = File::read("../../../libs/bindgen/default/Windows.winmd").unwrap();

    for def in file.table::<TypeDef>() {
        if def.name() == "IStringable" {
            break;
        }
    }
}
