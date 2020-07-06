use super::context;

pub fn build(middle: &context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = std::process::Command::new(&middle.config.binary_cc);
    cmd.arg("-g")
        .arg("-w") // Disable all gcc warnings.
        .arg("-o")
        .arg(middle.path_output.to_str().unwrap())
        .arg(middle.path_object.to_str().unwrap())
        .arg(middle.path_c.to_str().unwrap());
    match middle.config.platform {
        context::Platform::PosixX8664 => {
            cmd.arg(middle.path_prog.join("platform/posix_x86_64_runtime.S"));
        }
        context::Platform::PosixX8664Spectest => {
            cmd.arg(middle.path_prog.join("platform/posix_x86_64_spectest_runtime.S"));
        }
        context::Platform::PosixX8664Wasi => {
            cmd.arg(middle.path_prog.join("platform/posix_x86_64_wasi_runtime.S"));
        }
        _ => panic!("unreachable"),
    }

    cmd.spawn()?.wait()?;
    Ok(())
}
