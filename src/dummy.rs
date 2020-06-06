use super::context;
use std::io::Write;

pub fn init(middle: &mut context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    middle.dummy = middle.prog_dir.join(middle.file_stem.clone() + ".c");
    Ok(())
}

pub fn gcc_build(middle: &context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    let output_bin = middle
        .prog_dir
        .join(middle.file_stem.clone())
        .to_str()
        .unwrap()
        .to_string();
    let mut cmd = std::process::Command::new(&middle.config.cc_binary);
    cmd.arg("-g")
        .arg("-w") // Disable all gcc warnings.
        .arg("-o")
        .arg(output_bin)
        .arg(middle.aot_object.to_str().unwrap())
        .arg(middle.dummy.to_str().unwrap());
    cmd.spawn()?.wait()?;
    Ok(())
}

pub fn run(
    middle: &context::Middle,
) -> Result<std::process::ExitStatus, Box<dyn std::error::Error>> {
    let mut cmd = std::process::Command::new(
        middle
            .prog_dir
            .join(middle.file_stem.clone())
            .to_str()
            .unwrap(),
    );
    rog::debugln!("{:?}", cmd);
    Ok(cmd.spawn()?.wait()?)
}
