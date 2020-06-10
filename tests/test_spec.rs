use wasc::abi;
use wasc::aot_generator;
use wasc::code_builder;
use wasc::context;
use wasc::dummy;
use wasc::wavm;

mod misc;

fn test_spec_single_test<P: AsRef<std::path::Path>>(
    wasm_path: P,
    commands: Vec<serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = wasc::context::Config::default();
    config.abi = context::Abi::Spectest;
    config.wavm_binary = "./third_party/WAVM/build/bin/wavm".to_string();
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

    if middle.misc_has_init {
        dummy_file.write_line("init();")?;
    }
    let mut wavm_ret_index = 1;
    let mut uint32_t_index = 1;
    let mut uint64_t_index = 1;
    for command in commands {
        match command["type"].as_str().unwrap() {
            "assert_return" | "action" => {
                let action = command["action"].as_object().unwrap();
                let ty = action["type"].as_str().unwrap();

                match ty {
                    "invoke" => {
                        let field: &str = action["field"].as_str().unwrap();
                        let args = action["args"].as_array().unwrap();
                        let expected = command["expected"].as_array().unwrap();

                        let mut args_with_null = vec!["NULL".to_string()];
                        for e in args {
                            match e["type"].as_str().unwrap() {
                                "i32" => {
                                    args_with_null.push(e["value"].as_str().unwrap().to_string());
                                }
                                "i64" => {
                                    args_with_null.push(e["value"].as_str().unwrap().to_string());
                                }
                                "f32" => {
                                    dummy_file.write_line(
                                        format!(
                                            "uint32_t u32_{} = {};",
                                            uint32_t_index,
                                            e["value"].as_str().unwrap()
                                        )
                                        .as_str(),
                                    )?;
                                    args_with_null
                                        .push(format!("*(float *)&u32_{}", uint32_t_index));
                                    uint32_t_index += 1;
                                }
                                "f64" => {
                                    dummy_file.write_line(
                                        format!(
                                            "uint64_t u64_{} = {};",
                                            uint64_t_index,
                                            e["value"].as_str().unwrap()
                                        )
                                        .as_str(),
                                    )?;
                                    args_with_null
                                        .push(format!("*(double *)&u64_{}", uint64_t_index));
                                    uint64_t_index += 1;
                                }
                                _ => unimplemented!(),
                            }
                        }

                        if expected.len() != 0 {
                            let rttype = match expected[0]["type"].as_str().unwrap() {
                                "i32" => "wavm_ret_int32_t",
                                "i64" => "wavm_ret_int64_t",
                                "f32" => "wavm_ret_float",
                                "f64" => "wavm_ret_double",
                                _ => unimplemented!(),
                            };
                            dummy_file.write_line(
                                format!(
                                    "{} wavm_ret{} = wavm_exported_function_{}({});",
                                    rttype,
                                    wavm_ret_index,
                                    aot_generator::convert_func_name_to_c_function(field),
                                    args_with_null.join(",")
                                )
                                .as_str(),
                            )?;

                            match expected[0]["type"].as_str().unwrap() {
                                "i32" => {
                                    dummy_file.write_line(
                                        format!(
                                            "if (*(uint32_t *)&wavm_ret{}.value != {}) {{",
                                            wavm_ret_index,
                                            expected[0]["value"].as_str().unwrap()
                                        )
                                        .as_str(),
                                    )?;
                                }
                                "i64" => {
                                    dummy_file.write_line(
                                        format!(
                                            "if (*(uint64_t *)&wavm_ret{}.value != {}) {{",
                                            wavm_ret_index,
                                            expected[0]["value"].as_str().unwrap()
                                        )
                                        .as_str(),
                                    )?;
                                }
                                "f32" => {
                                    let r_str: &str = expected[0]["value"].as_str().unwrap();
                                    if r_str.starts_with("nan") {
                                        dummy_file.write_line(
                                            format!(
                                                "if (wavm_ret{}.value == wavm_ret{}.value) {{",
                                                wavm_ret_index, wavm_ret_index,
                                            )
                                            .as_str(),
                                        )?;
                                    } else {
                                        dummy_file.write_line(
                                            format!(
                                                "if (*(uint32_t *)&wavm_ret{}.value != {}) {{",
                                                wavm_ret_index,
                                                expected[0]["value"].as_str().unwrap()
                                            )
                                            .as_str(),
                                        )?;
                                    }
                                }
                                "f64" => {
                                    let r_str: &str = expected[0]["value"].as_str().unwrap();
                                    if r_str.starts_with("nan") {
                                        dummy_file.write_line(
                                            format!(
                                                "if (wavm_ret{}.value == wavm_ret{}.value) {{",
                                                wavm_ret_index, wavm_ret_index,
                                            )
                                            .as_str(),
                                        )?;
                                    } else {
                                        dummy_file.write_line(
                                            format!(
                                                "if (*(uint64_t *)&wavm_ret{}.value != {}) {{",
                                                wavm_ret_index,
                                                expected[0]["value"].as_str().unwrap(),
                                            )
                                            .as_str(),
                                        )?;
                                    }
                                }
                                _ => unimplemented!(),
                            }
                            dummy_file.intend();
                            dummy_file
                                .write_line(format!("return {};", wavm_ret_index).as_str())?;
                            dummy_file.extend();
                            dummy_file.write_line("}")?;
                            wavm_ret_index += 1;
                        } else {
                            dummy_file.write_line(
                                format!(
                                    "wavm_exported_function_{}({});",
                                    aot_generator::convert_func_name_to_c_function(field),
                                    args_with_null.join(", ")
                                )
                                .as_str(),
                            )?;
                        }
                        dummy_file.write_line("")?;
                    }
                    _ => unimplemented!(),
                }
            }
            "assert_trap" => {
                // TODO
            }
            "assert_malformed" => {
                // TODO
            }
            "assert_invalid" => {
                // TODO
            }
            "assert_unlinkable" => {
                // TODO
            }
            "assert_exhaustion" => {
                // TODO
            }
            "assert_uninstantiable" => {
                // TODO
            }
            "register" => {
                // TODO
            }
            _ => unimplemented!(),
        }
    }
    dummy_file.extend();
    dummy_file.write_line("}")?;

    dummy::gcc_build(&middle)?;

    let exit_status = dummy::run(&middle)?;
    rog::debugln!("{:?} {}", middle.dummy, exit_status);
    assert!(exit_status.success());
    Ok(())
}

fn test_spec_single_suit<P: AsRef<std::path::Path>>(
    spec_path: P,
    skip: Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec_path = spec_path.as_ref();
    let file_stem = spec_path.file_stem().unwrap().to_str().unwrap();
    let path_json = spec_path.join(format!("{}.json", file_stem));
    let file_json = std::fs::File::open(&path_json).unwrap();
    let json: serde_json::Value =
        serde_json::from_reader(std::io::BufReader::new(&file_json)).unwrap();

    let mut wasm_file = std::path::PathBuf::new();
    let mut commands: Vec<serde_json::Value> = vec![];

    for command in json["commands"].as_array().unwrap() {
        match command["type"].as_str().unwrap() {
            "module" => {
                if wasm_file.to_str().unwrap() != "" {
                    test_spec_single_test(&wasm_file, commands.clone())?;
                    commands.clear();
                }
                let file_name: &str = command["filename"].as_str().unwrap();
                let nice_name: &str = &file_name.replacen(".", "_", 1);
                if skip.contains(&nice_name) {
                    rog::debugln!("skip {:?}", nice_name);
                    wasm_file = std::path::PathBuf::new();
                } else {
                    wasm_file = spec_path.join(&nice_name);
                }
            }
            _ => {
                commands.push(command.clone());
            }
        }
    }
    if wasm_file.to_str().unwrap() != "" {
        test_spec_single_test(&wasm_file, commands.clone())?;
        commands.clear();
    }
    Ok(())
}

#[test]
fn test_spec() {
    misc::open_log();
    let wasc_path = std::path::PathBuf::from("./res/spectest_wasc");
    if wasc_path.exists() {
        std::fs::remove_dir_all(&wasc_path).unwrap();
    }
    std::fs::create_dir(&wasc_path).unwrap();
    let spec_path = std::path::PathBuf::from("./res/spectest");
    for d_path in spec_path.read_dir().unwrap() {
        let d_pbuf = d_path.unwrap().path();
        let d_file_name = d_pbuf.file_name().unwrap().to_str().unwrap();
        std::fs::create_dir(wasc_path.join(&d_file_name)).unwrap();
        for f_path in d_pbuf.read_dir().unwrap() {
            let f_pbuf = f_path.unwrap().path();
            let f_file_stem = f_pbuf.file_stem().unwrap().to_str().unwrap();
            let f_nice_stem = f_file_stem.replace(".", "_");
            let f_file_name = f_nice_stem + "." + f_pbuf.extension().unwrap().to_str().unwrap();
            std::fs::copy(f_pbuf, wasc_path.join(&d_file_name).join(&f_file_name)).unwrap();
        }
    }

    test_spec_single_suit("./res/spectest_wasc/address", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/align", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/binary", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/binary-leb128", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/br_if", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/br_table", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/break-drop", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/comments", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/const", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/custom", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/data", vec![]).unwrap();
    test_spec_single_suit(
        "./res/spectest_wasc/elem",
        vec!["elem_39.wasm", "elem_40.wasm"],
    )
    .unwrap();
    test_spec_single_suit("./res/spectest_wasc/endianness", vec![]).unwrap();
    // test_spec_single_suit("./res/spectest_wasc/exports").unwrap();
    test_spec_single_suit("./res/spectest_wasc/f32", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/f32_bitwise", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/f32_cmp", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/f64", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/f64_bitwise", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/f64_cmp", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/float_exprs", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/float_literals", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/float_memory", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/float_misc", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/forward", vec![]).unwrap();
    // test_spec_single_suit("./res/spectest_wasc/func_ptrs").unwrap();
    test_spec_single_suit("./res/spectest_wasc/global", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/globals", vec![]).unwrap();
    // test_spec_single_suit("./res/spectest_wasc/imports").unwrap();
    test_spec_single_suit("./res/spectest_wasc/inline-module", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/int_exprs", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/int_literals", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/labels", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/left-to-right", vec![]).unwrap();
    // test_spec_single_suit("./res/spectest_wasc/linking").unwrap(); // skip
    // test_spec_single_suit("./res/spectest_wasc/load").unwrap();
    test_spec_single_suit("./res/spectest_wasc/local_get", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/local_set", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/local_tee", vec![]).unwrap(); // tableReferenceBias
    test_spec_single_suit("./res/spectest_wasc/memory", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/memory_grow", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/memory_redundancy", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/memory_size", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/memory_trap", vec![]).unwrap();
    // test_spec_single_suit("./res/spectest_wasc/names").unwrap();
    test_spec_single_suit("./res/spectest_wasc/nop", vec![]).unwrap(); // tableReferenceBias
    test_spec_single_suit("./res/spectest_wasc/return", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/select", vec![]).unwrap(); // tableReferenceBias
    // test_spec_single_suit("./res/spectest_wasc/skip-stack-guard-page").unwrap();
    test_spec_single_suit("./res/spectest_wasc/stack", vec![]).unwrap();
    // test_spec_single_suit("./res/spectest_wasc/start").unwrap();
    test_spec_single_suit("./res/spectest_wasc/store", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/switch", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/table", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/token", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/traps", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/type", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/typecheck", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/unreachable", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/unreached-invalid", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/unwind", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/utf8-custom-section-id", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/utf8-import-field", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/utf8-import-module", vec![]).unwrap();
    test_spec_single_suit("./res/spectest_wasc/utf8-invalid-encoding", vec![]).unwrap();
}

#[test]
fn test_once() {
    misc::open_log();
    let wasc_path = std::path::PathBuf::from("./res/spectest_wasc");
    if wasc_path.exists() {
        std::fs::remove_dir_all(&wasc_path).unwrap();
    }
    std::fs::create_dir(&wasc_path).unwrap();
    let spec_path = std::path::PathBuf::from("./res/spectest");
    for d_path in spec_path.read_dir().unwrap() {
        let d_pbuf = d_path.unwrap().path();
        let d_file_name = d_pbuf.file_name().unwrap().to_str().unwrap();
        std::fs::create_dir(wasc_path.join(&d_file_name)).unwrap();
        for f_path in d_pbuf.read_dir().unwrap() {
            let f_pbuf = f_path.unwrap().path();
            let f_file_stem = f_pbuf.file_stem().unwrap().to_str().unwrap();
            let f_nice_stem = f_file_stem.replace(".", "_");
            let f_file_name = f_nice_stem + "." + f_pbuf.extension().unwrap().to_str().unwrap();
            std::fs::copy(f_pbuf, wasc_path.join(&d_file_name).join(&f_file_name)).unwrap();
        }
    }
    // test_spec_single_suit("./res/spectest_wasc/exports").unwrap();
    // test_spec_single_suit("./res/spectest_wasc/func_ptrs").unwrap();
    // test_spec_single_suit("./res/spectest_wasc/imports").unwrap();
    // test_spec_single_suit("./res/spectest_wasc/linking").unwrap(); // skip
    // test_spec_single_suit("./res/spectest_wasc/load").unwrap(); // memory.grow
    // test_spec_single_suit("./res/spectest_wasc/names").unwrap();
    // test_spec_single_suit("./res/spectest_wasc/skip-stack-guard-page").unwrap(); // wavm_probe_stack
    // test_spec_single_suit("./res/spectest_wasc/start").unwrap();
}
