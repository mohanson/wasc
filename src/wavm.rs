use super::context;

pub fn compile(middle: &mut context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    let outwasm = middle.prog_dir.join(middle.file_stem.clone() + "_precompiled.wasm");
    let mut cmd = std::process::Command::new(&middle.config.binary_wavm);
    cmd.arg("compile")
        .arg("--enable")
        .arg("all")
        .arg(middle.file.clone())
        .arg(outwasm.to_str().unwrap());
    cmd.spawn()?.wait()?;
    middle.wavm_precompiled_wasm = outwasm;
    Ok(())
}
