use super::context;

pub fn init(middle: &mut context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    middle.dummy = middle.path_prog.join(middle.file_stem.clone() + ".c");
    Ok(())
}

pub fn gcc_build(middle: &context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    let output_bin = middle
        .path_prog
        .join(middle.file_stem.clone())
        .to_str()
        .unwrap()
        .to_string();
    let mut cmd = std::process::Command::new(&middle.config.binary_cc);
    cmd.arg("-g")
        .arg("-w") // Disable all gcc warnings.
        .arg("-o")
        .arg(output_bin)
        .arg(middle.aot_object.to_str().unwrap())
        .arg(middle.dummy.to_str().unwrap());
    match middle.config.platform {
        context::Platform::PosixX8664 => {
            cmd.arg(
                middle
                    .path_prog
                    .join(format!("{}_platform/posix_x86_64_runtime.S", middle.file_stem)),
            );
        }
        context::Platform::PosixX8664Spectest => {
            cmd.arg(
                middle
                    .path_prog
                    .join(format!("{}_platform/posix_x86_64_spectest_runtime.S", middle.file_stem)),
            );
        }
        context::Platform::PosixX8664Wasi => {
            cmd.arg(
                middle
                    .path_prog
                    .join(format!("{}_platform/posix_x86_64_wasi_runtime.S", middle.file_stem)),
            );
        }
        _ => panic!("unreachable"),
    }

    cmd.spawn()?.wait()?;
    Ok(())
}

pub fn run(middle: &context::Middle) -> Result<std::process::ExitStatus, Box<dyn std::error::Error>> {
    let mut cmd = std::process::Command::new(middle.path_prog.join(middle.file_stem.clone()).to_str().unwrap());
    rog::debugln!("{:?}", cmd);
    Ok(cmd.spawn()?.wait()?)
}
