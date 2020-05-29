const WAVM_BINARY: &str = "wavm";

fn wavm_compile<P: AsRef<std::path::Path>>(source: P) {
    let source_path = source.as_ref();
    let parent = source_path.parent().unwrap();
    let file_stem = source_path.file_stem().unwrap();
    rog::debugln!(
        "wavm_compile source_path={:?} file_stem={:?}",
        source_path,
        file_stem
    );
    let dest_path = parent.join(file_stem).with_extension("wasm");
    rog::debugln!("wavm_compile dest_path={:?}", dest_path);

    let mut cmd = std::process::Command::new(WAVM_BINARY);
    cmd.arg("compile")
        .arg("--enable")
        .arg("all")
        .arg(source_path.to_str().unwrap())
        .arg(dest_path.to_str().unwrap());
    rog::debugln!("wavm_compile exec {:?}", cmd);
    cmd.spawn().unwrap().wait().unwrap();
}

fn main() {
    rog::reg("wasc");
    wavm_compile("examples/fib.wat");
}
