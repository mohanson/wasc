use wasc::aot_generator;
use wasc::code_builder;
use wasc::compile;
use wasc::context;
use wasc::dummy;

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

    let ep_path = middle.path_prog.join(format!("{}.c", middle.file_stem));
    let mut ep_file = code_builder::CodeBuilder::place(ep_path);
    let platform_header = match middle.config.platform {
        context::Platform::PosixX8664 => format!("./{}_platform/posix_x86_64.h", middle.file_stem),
        context::Platform::PosixX8664Spectest => format!("./{}_platform/posix_x86_64_spectest.h", middle.file_stem),
        context::Platform::PosixX8664Wasi => format!("./{}_platform/posix_x86_64_wasi.h", middle.file_stem),
        context::Platform::Unknown => panic!("unreachable"),
    };
    ep_file.write(format!("#include \"{}_glue.h\"", middle.file_stem).as_str());
    ep_file.write(format!("#include \"{}\"", platform_header));
    ep_file.close()?;

    dummy::init(&mut middle)?;
    dummy::gcc_build(&middle)?;

    Ok(())
}
