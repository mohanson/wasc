use super::context;

pub fn init(middle: &mut context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    let platform_code_path = middle.prog_dir.join(middle.file_stem.clone() + "_platform");
    std::fs::create_dir(&platform_code_path)?;
    match middle.config.platform {
        context::Platform::PosixX8664 => {
            let path_header = platform_code_path.join("posix_x86_64.h");
            std::fs::write(&path_header, &middle.config.platform_posix_x86_64_spectest)?;
            let path_s = platform_code_path.join("posix_x86_64_runtime.S");
            std::fs::write(&path_s, &middle.config.platform_posix_x86_64_spectest_runtime)?;
        }
        context::Platform::PosixX8664Spectest => {
            let path_header = platform_code_path.join("posix_x86_64_spectest.h");
            std::fs::write(&path_header, &middle.config.platform_posix_x86_64_spectest)?;
            let path_s = platform_code_path.join("posix_x86_64_spectest_runtime.S");
            std::fs::write(&path_s, &middle.config.platform_posix_x86_64_spectest_runtime)?;
        }
        _ => unimplemented!(),
    }
    Ok(())
}
