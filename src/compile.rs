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
        .arg(middle.path_precompiled.to_str().unwrap());
    cmd_wavm.spawn()?.wait()?;

    // Init platform based code.
    rog::debugln!("create {}", middle.path_platform_code_folder.to_str().unwrap());
    std::fs::create_dir(&middle.path_platform_code_folder)?;
    match middle.config.platform {
        context::Platform::PosixX8664 => {
            rog::debugln!("create {:?}", middle.path_platform_header);
            std::fs::write(&middle.path_platform_header, &middle.config.platform_posix_x86_64)?;
            rog::debugln!("create {:?}", middle.path_platform_s);
            std::fs::write(
                &middle.path_platform_header,
                &middle.config.platform_posix_x86_64_runtime,
            )?;
        }
        context::Platform::PosixX8664Spectest => {
            rog::debugln!("create {:?}", middle.path_platform_header);
            std::fs::write(
                &middle.path_platform_header,
                &middle.config.platform_posix_x86_64_spectest,
            )?;
            rog::debugln!("create {:?}", middle.path_platform_s);
            std::fs::write(
                &middle.path_platform_s,
                &middle.config.platform_posix_x86_64_spectest_runtime,
            )?;
        }
        context::Platform::PosixX8664Wasi => {
            rog::debugln!("create {:?}", middle.path_platform_header);
            std::fs::write(&middle.path_platform_header, &middle.config.platform_posix_x86_64_wasi)?;
            rog::debugln!("create {:?}", middle.path_platform_s);
            std::fs::write(
                &middle.path_platform_s,
                &middle.config.platform_posix_x86_64_wasi_runtime,
            )?;
        }
        context::Platform::Unknown => {
            panic!("unreachable");
        }
    }

    Ok(middle)
}
