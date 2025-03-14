use windows_ecma335::reader::*;

#[test]
fn test() {
    let file = std::fs::read("../../../libs/bindgen/default/Windows.winmd").unwrap();
    let file = File::new(file).unwrap();

    for  def in file.table::<TypeDef>() {
        if def.name() == "IStringable" {

            break;
        }
    }
}
