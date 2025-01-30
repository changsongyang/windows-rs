use windows_ecma335::*;

#[test]
fn test() {
    let file = File::new("test");
    let bytes = file.into_stream();
    std::fs::write("tests/empty.winmd", bytes).unwrap();
}
