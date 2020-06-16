use wasc::aot_generator;
use wasc::code_builder;
use wasc::compile;
use wasc::context;
use wasc::dummy;

mod misc;

fn test_spec_single_test<P: AsRef<std::path::Path>>(wasm_path: P) -> Result<i32, Box<dyn std::error::Error>> {
    let mut config = wasc::context::Config::default();
    config.platform = context::Platform::PosixX8664;
    config.binary_wavm = String::from("./third_party/WAVM/build/bin/wavm");

    let mut middle = compile::compile(&wasm_path, config)?;
    aot_generator::generate(&mut middle)?;

    dummy::init(&mut middle)?;
    let mut dummy_file = code_builder::CodeBuilder::place(&middle.dummy);
    dummy_file.write(format!("#include \"{}_glue.h\"", middle.file_stem).as_str());
    dummy_file.write(format!("#include \"./{}_platform/posix_x86_64.h\"", middle.file_stem.clone()).as_str());
    dummy_file.write("");
    dummy_file.write("int main() {");
    dummy_file.write("wavm_ret_int32_t wavm_ret = wavm_exported_function_main(NULL);");
    dummy_file.write("return wavm_ret.value;");
    dummy_file.write("}");
    dummy_file.close()?;

    dummy::gcc_build(&middle)?;

    let exit_status = dummy::run(&middle)?;
    rog::debugln!("{:?} {}", wasm_path.as_ref(), exit_status);
    Ok(exit_status.code().unwrap())
}

#[test]
fn test_bugs() -> Result<(), Box<dyn std::error::Error>> {
    misc::open_log();
    let dest = std::path::Path::new("./res/spectest_bugs_wasc");
    if dest.exists() {
        std::fs::remove_dir_all(dest)?;
    }
    misc::copy_dir("./res/spectest_bugs", dest)?;

    let mut exit_code = 0;
    let _ = exit_code;
    exit_code = test_spec_single_test("./res/spectest_bugs_wasc/import_global.wasm")?;
    assert_eq!(exit_code, 42);
    // exit_code = test_spec_single_test("./res/spectest_bugs_wasc/import_global_2.wasm")?;
    // assert_eq!(exit_code, 52);
    Ok(())
}
