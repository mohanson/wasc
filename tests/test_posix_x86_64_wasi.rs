mod misc;

#[test]
fn test_posix_x86_64_wasi() -> Result<(), Box<dyn std::error::Error>> {
    misc::open_log();
    let dest = std::path::Path::new("./res/posix_x86_64_wasi");
    if dest.exists() {
        std::fs::remove_dir_all(dest)?;
    }
    misc::copy_dir("./res/wasi", dest)?;

    for wasm in std::fs::read_dir("./res/posix_x86_64_wasi").unwrap() {
        let e = wasm.unwrap();
        if !e.file_name().to_str().unwrap().ends_with(".wasm") {
            continue;
        }
        let mut cmd = std::process::Command::new("./build/wasc");
        cmd.arg(e.path().to_str().unwrap());
        rog::debugln!("$ {:?}", cmd);
        assert_eq!(cmd.spawn()?.wait()?.code().unwrap(), 0);
    }
    Ok(())
}
