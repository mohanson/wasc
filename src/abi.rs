use super::context;

pub fn init(middle: &mut context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    let abi_path = middle.prog_dir.join(middle.file_stem.clone() + "_abi");
    std::fs::create_dir(&abi_path)?;
    rog::debugln!("abi path={:?}", abi_path);
    let abi_file = abi_path.join("posix_wasi_abi.h");
    std::fs::write(&abi_file, &middle.config.abi_posix_wasi)?;
    middle.abi_path = abi_path;
    middle.abi_file = abi_file;
    Ok(())
}
