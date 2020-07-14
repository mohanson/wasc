use wasc::compile;
use wasc::context;
use wasc::gcc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Usage of wasc:
    //
    // wasc
    //     --gcc [GCC binary]
    //     -p --platform [PLATFORM]
    //     -s --save
    //     -v --verbose
    //     --wasm [WAVM binary]
    //     source [WASM/WA(S)T source file]
    //
    // PLATFORM:
    //   ckb_vm_assemblyscript
    //   ckb_vm_spectest
    //   posix_x86_64
    //   posix_x86_64_spectest
    //   posix_x86_64_wasi
    let mut fl_source = String::from("");
    let mut fl_platform = String::from("");
    let mut fl_wavm = String::from(
        std::env::current_exe()?
            .parent()
            .unwrap()
            .join("wavm")
            .to_str()
            .unwrap(),
    );
    let mut fl_gcc = String::from("");
    let mut fl_verbose = false;
    let mut fl_save = false;
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
        ap.refer(&mut fl_gcc).add_option(&["--gcc"], argparse::Store, "GCC");
        ap.refer(&mut fl_verbose)
            .add_option(&["-v", "--verbose"], argparse::StoreTrue, "");
        ap.refer(&mut fl_save)
            .add_option(&["-s", "--save"], argparse::StoreTrue, "save temporary files");
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
        "ckb_vm_assemblyscript" => context::Platform::CKBVMAssemblyScript,
        "ckb_vm_spectest" => context::Platform::CKBVMSpectest,
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
    if fl_gcc.is_empty() {
        match config.platform {
            context::Platform::CKBVMAssemblyScript | context::Platform::CKBVMSpectest => {
                fl_gcc = String::from("riscv64-unknown-elf-gcc");
            }
            context::Platform::PosixX8664
            | context::Platform::PosixX8664Spectest
            | context::Platform::PosixX8664Wasi => {
                fl_gcc = String::from("gcc");
            }
            _ => {}
        }
    }
    config.binary_wavm = fl_wavm;
    config.binary_cc = fl_gcc;

    let middle = compile::compile(&fl_source, config)?;

    gcc::build(&middle)?;

    std::fs::copy(
        middle.path_output.clone(),
        middle.file.parent().unwrap().join(middle.file_stem.clone()),
    )?;

    if !fl_save {
        rog::debugln!("remove {}", middle.path_prog.to_str().unwrap());
        std::fs::remove_dir_all(middle.path_prog)?;
    }
    Ok(())
}
