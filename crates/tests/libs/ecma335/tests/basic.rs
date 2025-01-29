use windows_ecma335::*;

#[test]
fn test() {
    let file = File::new("hello");
    let bytes = file.into_stream();
    std::fs::write("/git/test.winmd", bytes).unwrap();
}
