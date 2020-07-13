use super::aot_generator;
use super::code_builder;
use super::context;

// The main entry function for wasc compiler. It is expected that it will be a complete set of compilation work.
// TODO: only a small part is realized.
pub fn compile<P: AsRef<std::path::Path>>(
    path: P,
    config: context::Config,
) -> Result<context::Middle, Box<dyn std::error::Error>> {
    let mut middle = context::Middle::default();
    middle.init_config(config);
    middle.init_file(path);

    rog::debugln!("create {}", middle.path_prog.to_str().unwrap());
    if let Ok(()) = std::fs::create_dir(&middle.path_prog) {}

    // Get wavm precompiled module.
    let mut cmd_wavm = std::process::Command::new(&middle.config.binary_wavm);
    cmd_wavm.arg("compile").arg("--enable").arg("all");
    match middle.config.platform {
        context::Platform::CKBVMSpectest => {
            cmd_wavm.arg("--target-triple").arg("riscv64");
        }
        _ => {}
    }
    cmd_wavm
        .arg(middle.file.clone())
        .arg(middle.path_precompiled.to_str().unwrap());
    rog::debugln!("$ {:?}", cmd_wavm);
    let exit_status = cmd_wavm.spawn()?.wait()?;
    if !exit_status.success() {
        std::process::exit(exit_status.code().unwrap());
    }
    // Init platform based code.
    rog::debugln!("create {}", middle.path_platform_code_folder.to_str().unwrap());
    if let Ok(()) = std::fs::create_dir(&middle.path_platform_code_folder) {}
    rog::debugln!("create {}", middle.path_platform_common_code_folder.to_str().unwrap());
    if let Ok(()) = std::fs::create_dir(&middle.path_platform_common_code_folder) {}
    rog::debugln!("create {}", &middle.path_platform_common_wavm_h.to_str().unwrap());
    std::fs::write(
        &middle.path_platform_common_wavm_h,
        &middle.config.platform_common_wavm_h,
    )?;
    match middle.config.platform {
        context::Platform::CKBVMAssemblyScript => {
            rog::debugln!("create {}", middle.path_platform_header.to_str().unwrap());
            std::fs::write(
                &middle.path_platform_header,
                &middle.config.platform_ckb_vm_assemblyscript_h,
            )?;
            rog::debugln!(
                "create {}",
                middle.path_platform_lds.to_owned().unwrap().to_str().unwrap()
            );
            std::fs::write(
                &middle.path_platform_lds.to_owned().unwrap(),
                &middle.config.platform_ckb_vm_assemblyscript_lds,
            )?;
            rog::debugln!("create {}", middle.path_platform_s.to_str().unwrap());
            std::fs::write(
                &middle.path_platform_s,
                &middle.config.platform_ckb_vm_assemblyscript_runtime_s,
            )?;
        }
        context::Platform::CKBVMSpectest => {
            rog::debugln!("create {}", middle.path_platform_header.to_str().unwrap());
            std::fs::write(&middle.path_platform_header, &middle.config.platform_ckb_vm_spectest_h)?;
            rog::debugln!(
                "create {}",
                middle.path_platform_lds.to_owned().unwrap().to_str().unwrap()
            );
            std::fs::write(
                &middle.path_platform_lds.to_owned().unwrap(),
                &middle.config.platform_ckb_vm_spectest_lds,
            )?;
            rog::debugln!("create {}", middle.path_platform_s.to_str().unwrap());
            std::fs::write(
                &middle.path_platform_s,
                &middle.config.platform_ckb_vm_spectest_runtime_s,
            )?;
        }
        context::Platform::PosixX8664 => {
            rog::debugln!("create {}", middle.path_platform_header.to_str().unwrap());
            std::fs::write(&middle.path_platform_header, &middle.config.platform_posix_x86_64_h)?;
            rog::debugln!("create {}", middle.path_platform_s.to_str().unwrap());
            std::fs::write(&middle.path_platform_s, &middle.config.platform_posix_x86_64_runtime_s)?;
        }
        context::Platform::PosixX8664Spectest => {
            rog::debugln!("create {}", middle.path_platform_header.to_str().unwrap());
            std::fs::write(
                &middle.path_platform_header,
                &middle.config.platform_posix_x86_64_spectest_h,
            )?;
            rog::debugln!("create {}", middle.path_platform_s.to_str().unwrap());
            std::fs::write(
                &middle.path_platform_s,
                &middle.config.platform_posix_x86_64_spectest_runtime_s,
            )?;
        }
        context::Platform::PosixX8664Wasi => {
            rog::debugln!("create {}", middle.path_platform_header.to_str().unwrap());
            std::fs::write(
                &middle.path_platform_header,
                &middle.config.platform_posix_x86_64_wasi_h,
            )?;
            rog::debugln!("create {}", middle.path_platform_s.to_str().unwrap());
            std::fs::write(
                &middle.path_platform_s,
                &middle.config.platform_posix_x86_64_wasi_runtime_s,
            )?;
            rog::debugln!("create {}", &middle.path_platform_common_wasi_h.to_str().unwrap());
            std::fs::write(
                &middle.path_platform_common_wasi_h,
                &middle.config.platform_common_wasi_h,
            )?;
        }
        context::Platform::Unknown => {
            panic!("unreachable");
        }
    }

    // AOT generator
    aot_generator::generate(&mut middle)?;

    let mut main_file = code_builder::CodeBuilder::create(&middle.path_c);
    let platform_header = match middle.config.platform {
        context::Platform::CKBVMAssemblyScript => "platfrom/ckb_vm_assemblyscript.h",
        context::Platform::CKBVMSpectest => "platform/ckb_vm_spectest.h",
        context::Platform::PosixX8664 => "platform/posix_x86_64.h",
        context::Platform::PosixX8664Spectest => "platform/posix_x86_64_spectest.h",
        context::Platform::PosixX8664Wasi => "platform/posix_x86_64_wasi.h",
        context::Platform::Unknown => panic!("unreachable"),
    };
    main_file.write(format!("#include \"{}_glue.h\"", middle.file_stem).as_str());
    main_file.write(format!("#include \"{}\"", platform_header));
    main_file.close()?;

    Ok(middle)
}
