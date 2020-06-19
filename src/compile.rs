use super::context;

// The main entry function for wasc compiler. It is expected that it will be a complete set of compilation work.
pub fn compile<P: AsRef<std::path::Path>>(
    path: P,
    config: context::Config,
) -> Result<context::Middle, Box<dyn std::error::Error>> {
    let mut middle = context::Middle::default();
    middle.init_config(config);
    middle.init_file(path);

    // Get wavm precompiled module.
    let mut cmd_wavm = std::process::Command::new(&middle.config.binary_wavm);
    cmd_wavm
        .arg("compile")
        .arg("--enable")
        .arg("all")
        .arg(middle.file.clone())
        .arg(middle.wavm_precompiled_wasm.to_str().unwrap());
    cmd_wavm.spawn()?.wait()?;

    // Init platform based code.
    rog::debugln!("create {}", middle.platform_code_path.to_str().unwrap());
    std::fs::create_dir(&middle.platform_code_path)?;
    match middle.config.platform {
        context::Platform::PosixX8664 => {
            let path_header = middle.platform_code_path.join("posix_x86_64.h");
            rog::debugln!("create {}", path_header.to_str().unwrap());
            std::fs::write(&path_header, &middle.config.platform_posix_x86_64_spectest)?;
            let path_s = middle.platform_code_path.join("posix_x86_64_runtime.S");
            rog::debugln!("create {}", path_s.to_str().unwrap());
            std::fs::write(&path_s, &middle.config.platform_posix_x86_64_spectest_runtime)?;
        }
        context::Platform::PosixX8664Spectest => {
            let path_header = middle.platform_code_path.join("posix_x86_64_spectest.h");
            rog::debugln!("create {}", path_header.to_str().unwrap());
            std::fs::write(&path_header, &middle.config.platform_posix_x86_64_spectest)?;
            let path_s = middle.platform_code_path.join("posix_x86_64_spectest_runtime.S");
            rog::debugln!("create {}", path_s.to_str().unwrap());
            std::fs::write(&path_s, &middle.config.platform_posix_x86_64_spectest_runtime)?;
        }
        context::Platform::PosixX8664Wasi => {
            let path_header = middle.platform_code_path.join("posix_x86_64_wasi.h");
            rog::debugln!("create {}", path_header.to_str().unwrap());
            std::fs::write(&path_header, &middle.config.platform_posix_x86_64_wasi)?;
            let path_s = middle.platform_code_path.join("posix_x86_64_wasi_runtime.S");
            rog::debugln!("create {}", path_s.to_str().unwrap());
            std::fs::write(&path_s, &middle.config.platform_posix_x86_64_wasi_runtime)?;
        }
        context::Platform::Unknown => {
            // Must specify the target platform in advance, from environment variables, or command line parameters,
            // or guess.
            panic!("unknown platform");
        }
    }

    Ok(middle)
}
