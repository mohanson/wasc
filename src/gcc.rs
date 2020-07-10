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
        context::Platform::CKBVMSpectest => {
            cmd.arg(middle.path_prog.join("platform/ckb_vm_spectest_runtime.S"));
            cmd.arg("-Wl,-T");
            cmd.arg(middle.path_prog.join("platform/ckb_vm_spectest.lds"));
        }
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
    rog::debugln!("$ {:?}", cmd);
    let exit_code = cmd.spawn()?.wait()?;
    if exit_code.code().unwrap() != 0 {
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Interrupted, "")));
    }
    Ok(())
}
