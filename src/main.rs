use wasc::context;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    rog::reg("wasc");
    rog::reg("wasc::aot_generator");
    rog::reg("wasc::dummy");
    rog::reg("wasc::wavm");

    let mut source = String::from("");
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("WASC: WebAssembly native compilter");
        ap.refer(&mut source)
            .add_argument("source", argparse::Store, "WASM/WA(S)T source file");
        ap.parse_args_or_exit();
    }
    if source.is_empty() {
        rog::println!("wasc: missing file operand");
        std::process::exit(0);
    }
    let mut config = context::Config::default();
    config.wavm_binary = String::from("/src/wasc/third_party/WAVM/build/bin/wavm");

    let mut middle = context::Middle::default();
    middle.config = config;
    middle.dir = std::env::current_dir()?;
    middle.init_file(source);

    rog::debugln!("The wasc cli is work in progress.");

    // wavm::compile(&mut middle)?;
    // aot_generator::glue(&mut middle)?;
    // dummy::init(&mut middle)?;
    // let mut dummy_file = dummy::CodeBuilder::open(&middle.dummy)?;
    // dummy_file.write_line(format!("#include \"{}_glue.h\"", middle.file_stem).as_str())?;
    // dummy_file.write_line("#include \"abi/posix_wasi_abi.h\"")?;

    // dummy::gcc_build(&middle)?;

    Ok(())
}
