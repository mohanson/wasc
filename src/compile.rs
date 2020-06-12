use super::context;

pub fn compile<P: AsRef<std::path::Path>>(path: P, config: context::Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut middle = context::Middle::default();
    middle.init_config(config);
    middle.init_file(path);
    Ok(())
}
