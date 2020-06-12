use wasc::abi;
use wasc::aot_generator;
use wasc::code_builder;
use wasc::context;
use wasc::dummy;
use wasc::wavm;

mod misc;

fn test_spec_single_test<P: AsRef<std::path::Path>>(
    wasm_path: P,
) -> Result<i32, Box<dyn std::error::Error>> {
    let mut config = wasc::context::Config::default();
    config.platform = context::Platform::PosixX8664Spectest;
    config.binary_wavm = "./third_party/WAVM/build/bin/wavm".to_string();
    let mut middle = context::Middle::default();
    middle.config = config;
    middle.dir = std::env::current_dir()?;
    let wasm_path = wasm_path.as_ref();
    middle.init_file(&wasm_path);

    wavm::compile(&mut middle).unwrap();
    aot_generator::glue(&mut middle)?;
    abi::init(&mut middle)?;

    dummy::init(&mut middle)?;
    let mut dummy_file = code_builder::CodeBuilder::open(&middle.dummy)?;
    dummy_file.write_line(format!("#include \"{}_glue.h\"", middle.file_stem).as_str())?;
    dummy_file.write_line(
        format!("#include \"./{}_abi/spectest.h\"", middle.file_stem.clone()).as_str(),
    )?;
    dummy_file.write_line("")?;
    dummy_file.write_line("int main() {")?;
    dummy_file.intend();
    dummy_file.write_line("wavm_ret_int32_t wavm_ret = wavm_exported_function_main(NULL);")?;
    dummy_file.write_line("return wavm_ret.value;")?;
    dummy_file.extend();
    dummy_file.write_line("}")?;
    dummy::gcc_build(&middle)?;

    let exit_status = dummy::run(&middle)?;
    rog::debugln!("{:?} {}", wasm_path, exit_status);
    Ok(exit_status.code().unwrap())
}

#[test]
fn test_bugs() {
    misc::open_log();
    let dest = std::path::Path::new("./res/spectest_bugs_wasc");
    if dest.exists() {
        std::fs::remove_dir_all(dest).unwrap();
    }
    misc::copy_dir("./res/spectest_bugs", dest).unwrap();

    assert_eq!(
        test_spec_single_test("./res/spectest_bugs_wasc/import_global.wasm").unwrap(),
        42
    );
}
