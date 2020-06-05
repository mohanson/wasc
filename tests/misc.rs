#[allow(dead_code)]
pub fn copy_dir<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(
    from: P,
    to: Q,
) -> std::io::Result<()> {
    let src = from.as_ref();
    let dst = to.as_ref();
    std::fs::create_dir(dst)?;
    for path in src.read_dir()? {
        let pbuf = path.unwrap().path();
        let file_name = pbuf.file_name().unwrap().to_str().unwrap().to_string();
        if pbuf.is_dir() {
            copy_dir(pbuf, dst.join(file_name))?;
        } else {
            std::fs::copy(pbuf, dst.join(file_name))?;
        }
    }
    Ok(())
}

pub fn open_log() {
    // rog::reg("wasc::aot_generator");
    // rog::reg("wasc::dummy");
    // rog::reg("wasc::wavm");
    rog::reg("test_bugs");
    rog::reg("test_spec");
}
