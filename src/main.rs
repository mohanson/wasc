use wasc::aot_generator;
use wasc::compile;
use wasc::context;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    rog::reg("wasc");
    rog::reg("wasc::aot_generator");
    rog::reg("wasc::code_builder");
    rog::reg("wasc::compile");

    let mut source = String::from("");
    let mut platform = String::from("");
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("WASC: WebAssembly native compilter");
        ap.refer(&mut source)
            .add_argument("source", argparse::Store, "WASM/WA(S)T source file");
        ap.refer(&mut platform)
            .add_option(&["-p", "--platform"], argparse::Store, "");
        ap.parse_args_or_exit();
    }
    if source.is_empty() {
        rog::println!("wasc: missing file operand");
        std::process::exit(0);
    }
    rog::debugln!("The wasc cli is work in progress.");

    let mut config = context::Config::default();
    config.platform = match platform.as_str() {
        "posix_x86_64" => context::Platform::PosixX8664,
        "posix_x86_64_spectest" => context::Platform::PosixX8664Spectest,
        "posix_x86_64_wasi" => context::Platform::PosixX8664Wasi,
        _ => context::Platform::PosixX8664Wasi,
    };
    config.binary_wavm = String::from("/src/wasc/third_party/WAVM/build/bin/wavm");
    let mut middle = compile::compile(&source, config)?;
    aot_generator::generate(&mut middle)?;

    // aot_generator::glue(&mut middle)?;
    // dummy::init(&mut middle)?;
    // let mut dummy_file = dummy::CodeBuilder::open(&middle.dummy)?;
    // dummy_file.write_line(format!("#include \"{}_glue.h\"", middle.file_stem).as_str())?;
    // dummy_file.write_line("#include \"abi/posix_wasi_abi.h\"")?;
    // dummy::gcc_build(&middle)?;

    Ok(())
}
