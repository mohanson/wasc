use wasc::code_builder;
use wasc::compile;
use wasc::context;
use wasc::gcc;

mod misc;

fn test_single_test<P: AsRef<std::path::Path>>(wasm_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    let mut config = wasc::context::Config::default();
    config.platform = context::Platform::PosixX8664Spectest;
    config.binary_wavm = String::from("./third_party/WAVM/build/bin/wavm");
    let middle = compile::compile(&wasm_path, config)?;
    let mut dummy_file = code_builder::CodeBuilder::append(&middle.path_c)?;
    dummy_file.write("int main() {");
    dummy_file.write("wavm_ret_int32_t wavm_ret = wavm_exported_function_main(NULL);");
    dummy_file.write("return wavm_ret.value;");
    dummy_file.write("}");
    dummy_file.close()?;

    gcc::build(&middle)?;

    let mut cmd = std::process::Command::new(middle.path_prog.join(middle.file_stem.clone()).to_str().unwrap());
    let exit_status = cmd.spawn()?.wait()?;
    rog::println!("{:?} {}", wasm_path.as_ref(), exit_status);
    Ok(exit_status.code().unwrap())
}

#[test]
fn test_posix_x86_64_spectest_bugs() -> Result<(), Box<dyn std::error::Error>> {
    let dest = std::path::Path::new("./res/posix_x86_64_spectest_bugs");
    if dest.exists() {
        std::fs::remove_dir_all(dest)?;
    }
    misc::copy_dir("./res/spectest_bugs", dest)?;

    let mut exit_code = 0;
    let _ = exit_code;
    exit_code = test_single_test("./res/posix_x86_64_spectest_bugs/import_global.wasm")?;
    assert_eq!(exit_code, 42);
    exit_code = test_single_test("./res/posix_x86_64_spectest_bugs/import_global_add.wasm")?;
    assert_eq!(exit_code, 52);
    Ok(())
}
