use wasc::aot_generator;
use wasc::code_builder;
use wasc::compile;
use wasc::context;
use wasc::dummy;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fl_source = String::from("");
    let mut fl_platform = String::from("");
    let mut fl_wavm = String::from("wavm");
    let mut fl_verbose = false;
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("WASC: WebAssembly native compilter");
        ap.refer(&mut fl_source)
            .add_argument("source", argparse::Store, "WASM/WA(S)T source file");
        ap.refer(&mut fl_platform).add_option(
            &["-p", "--platform"],
            argparse::Store,
            "posix_x86_64 posix_x86_64_spectest posix_x86_64_wasi",
        );
        ap.refer(&mut fl_wavm)
            .add_option(&["--wavm"], argparse::Store, "WAVM binary");
        ap.refer(&mut fl_verbose)
            .add_option(&["-v", "--verbose"], argparse::StoreTrue, "");
        ap.parse_args_or_exit();
    }
    if fl_source.is_empty() {
        rog::println!("wasc: missing file operand");
        std::process::exit(1);
    }
    if fl_verbose {
        rog::reg("wasc");
        rog::reg("wasc::aot_generator");
        rog::reg("wasc::code_builder");
        rog::reg("wasc::compile");
    }

    let mut config = context::Config::default();
    config.platform = match fl_platform.as_str() {
        "posix_x86_64" => context::Platform::PosixX8664,
        "posix_x86_64_spectest" => context::Platform::PosixX8664Spectest,
        "posix_x86_64_wasi" => context::Platform::PosixX8664Wasi,
        "" => {
            if cfg!(unix) {
                context::Platform::PosixX8664Wasi
            } else {
                context::Platform::Unknown
            }
        }
        x => {
            rog::println!("wasc: unknown platform {}", x);
            std::process::exit(1);
        }
    };
    config.binary_wavm = fl_wavm;

    let mut middle = compile::compile(&fl_source, config)?;
    aot_generator::generate(&mut middle)?;

    let mut ep_file = code_builder::CodeBuilder::place(&middle.path_c);
    let platform_header = match middle.config.platform {
        context::Platform::PosixX8664 => "platform/posix_x86_64.h",
        context::Platform::PosixX8664Spectest => "platform/posix_x86_64_spectest.h",
        context::Platform::PosixX8664Wasi => "platform/posix_x86_64_wasi.h",
        context::Platform::Unknown => panic!("unreachable"),
    };
    ep_file.write(format!("#include \"{}_glue.h\"", middle.file_stem).as_str());
    ep_file.write(format!("#include \"{}\"", platform_header));
    ep_file.close()?;

    dummy::gcc_build(&middle)?;

    Ok(())
}
