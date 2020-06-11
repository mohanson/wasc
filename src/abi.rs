use super::context;

pub fn init(middle: &mut context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    let abi_path = middle.prog_dir.join(middle.file_stem.clone() + "_abi");
    std::fs::create_dir(&abi_path)?;
    rog::debugln!("abi path={:?}", abi_path);
    middle.abi_path = abi_path.clone();
    match middle.config.abi {
        context::Abi::Bare => {}
        context::Abi::Spectest => {
            let abi_path_header = abi_path.join("spectest.h");
            std::fs::write(&abi_path_header, &middle.config.abi_spectest)?;
            middle.abi_path_header = abi_path_header;
            let abi_path_s = abi_path.join("spectest_runtime.S");
            std::fs::write(&abi_path_s, &middle.config.abi_spectest_runtime)?;
            middle.abi_path_s = abi_path_s;
        }
    }
    Ok(())
}
