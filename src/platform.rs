use super::context;

pub fn init(middle: &mut context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    let abi_path = middle.prog_dir.join(middle.file_stem.clone() + "_platform");
    std::fs::create_dir(&abi_path)?;
    rog::debugln!("abi path={:?}", abi_path);
    middle.abi_path = abi_path.clone();
    match middle.config.platform {
        context::Platform::Unknown => {
            panic!("unknown");
        }
        context::Platform::PosixX8664 => {
            let abi_path_header = abi_path.join("posix_x86_64.h");
            std::fs::write(&abi_path_header, &middle.config.platform_posix_x86_64_spectest)?;
            middle.abi_path_header = abi_path_header;
            let abi_path_s = abi_path.join("posix_x86_64_runtime.S");
            std::fs::write(&abi_path_s, &middle.config.platform_posix_x86_64_spectest_runtime)?;
            middle.abi_path_s = abi_path_s;
        }
        context::Platform::PosixX8664Spectest => {
            let abi_path_header = abi_path.join("posix_x86_64_spectest.h");
            std::fs::write(&abi_path_header, &middle.config.platform_posix_x86_64_spectest)?;
            middle.abi_path_header = abi_path_header;
            let abi_path_s = abi_path.join("posix_x86_64_spectest_runtime.S");
            std::fs::write(&abi_path_s, &middle.config.platform_posix_x86_64_spectest_runtime)?;
            middle.abi_path_s = abi_path_s;
        }
    }
    Ok(())
}
