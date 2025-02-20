use windows_bindgen::*;

fn main() {
    let time = std::time::Instant::now();

    bindgen(["--etc", "crates/tools/bindings/src/async.txt"]).unwrap();
    bindgen(["--etc", "crates/tools/bindings/src/async_impl.txt"]).unwrap();
    bindgen(["--etc", "crates/tools/bindings/src/collections.txt"]).unwrap();
    bindgen(["--etc", "crates/tools/bindings/src/core_com.txt"]).unwrap();
    bindgen(["--etc", "crates/tools/bindings/src/core.txt"]).unwrap();
    bindgen(["--etc", "crates/tools/bindings/src/numerics.txt"]).unwrap();
    bindgen(["--etc", "crates/tools/bindings/src/registry.txt"]).unwrap();
    bindgen(["--etc", "crates/tools/bindings/src/result.txt"]).unwrap();
    bindgen(["--etc", "crates/tools/bindings/src/strings.txt"]).unwrap();
    bindgen(["--etc", "crates/tools/bindings/src/version.txt"]).unwrap();

    bindgen(["--etc", "crates/tools/bindings/src/sys.txt"]);
    bindgen(["--etc", "crates/tools/bindings/src/windows.txt"]);

    bindgen([
        "--etc",
        "crates/tools/bindings/src/metadata.txt",
        "--out",
        "crates/libs/bindgen/src/winmd/bindings.rs",
    ]).unwrap();

    bindgen([
        "--etc",
        "crates/tools/bindings/src/metadata.txt",
        "--out",
        "crates/libs/ecma335/src/bindings.rs",
    ]).unwrap();

    println!("Finished in {:.2}s", time.elapsed().as_secs_f32());
}
