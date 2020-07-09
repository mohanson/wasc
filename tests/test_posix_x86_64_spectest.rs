use wasc::aot_generator;
use wasc::code_builder;
use wasc::compile;
use wasc::context;
use wasc::gcc;

mod misc;

fn test_single_test<P: AsRef<std::path::Path>>(
    wasm_path: P,
    commands: Vec<serde_json::Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut config = wasc::context::Config::default();
    config.platform = context::Platform::PosixX8664Spectest;
    config.binary_wavm = "./third_party/WAVM/build/bin/wavm".to_string();

    let middle = compile::compile(&wasm_path, config)?;

    let mut ep_file = code_builder::CodeBuilder::place(&middle.path_c);
    ep_file.write(format!("#include \"{}_glue.h\"", middle.file_stem).as_str());
    ep_file.write("#include \"platform/posix_x86_64_spectest.h\"");
    ep_file.write("");
    ep_file.write("int main() {");
    ep_file.write("init();");
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
                                    ep_file.write(format!(
                                        "uint32_t u32_{} = {};",
                                        uint32_t_index,
                                        e["value"].as_str().unwrap()
                                    ));
                                    args_with_null.push(format!("*(float *)&u32_{}", uint32_t_index));
                                    uint32_t_index += 1;
                                }
                                "f64" => {
                                    ep_file.write(format!(
                                        "uint64_t u64_{} = {};",
                                        uint64_t_index,
                                        e["value"].as_str().unwrap()
                                    ));
                                    args_with_null.push(format!("*(double *)&u64_{}", uint64_t_index));
                                    uint64_t_index += 1;
                                }
                                _ => panic!("unreachable"),
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
                            ep_file.write(format!(
                                "{} wavm_ret{} = wavm_exported_function_{}({});",
                                rttype,
                                wavm_ret_index,
                                aot_generator::cnaming(field),
                                args_with_null.join(",")
                            ));

                            match expected[0]["type"].as_str().unwrap() {
                                "i32" => {
                                    ep_file.write(format!(
                                        "if (*(uint32_t *)&wavm_ret{}.value != {}) {{",
                                        wavm_ret_index,
                                        expected[0]["value"].as_str().unwrap()
                                    ));
                                }
                                "i64" => {
                                    ep_file.write(format!(
                                        "if (*(uint64_t *)&wavm_ret{}.value != {}) {{",
                                        wavm_ret_index,
                                        expected[0]["value"].as_str().unwrap()
                                    ));
                                }
                                "f32" => {
                                    let r_str: &str = expected[0]["value"].as_str().unwrap();
                                    if r_str.starts_with("nan") {
                                        ep_file.write(format!(
                                            "if (wavm_ret{}.value == wavm_ret{}.value) {{",
                                            wavm_ret_index, wavm_ret_index,
                                        ));
                                    } else {
                                        ep_file.write(format!(
                                            "if (*(uint32_t *)&wavm_ret{}.value != {}) {{",
                                            wavm_ret_index,
                                            expected[0]["value"].as_str().unwrap()
                                        ));
                                    }
                                }
                                "f64" => {
                                    let r_str: &str = expected[0]["value"].as_str().unwrap();
                                    if r_str.starts_with("nan") {
                                        ep_file.write(format!(
                                            "if (wavm_ret{}.value == wavm_ret{}.value) {{",
                                            wavm_ret_index, wavm_ret_index,
                                        ));
                                    } else {
                                        ep_file.write(format!(
                                            "if (*(uint64_t *)&wavm_ret{}.value != {}) {{",
                                            wavm_ret_index,
                                            expected[0]["value"].as_str().unwrap(),
                                        ));
                                    }
                                }
                                _ => panic!("unreachable"),
                            }
                            ep_file.write(format!("return {};", wavm_ret_index));
                            ep_file.write("}");
                            wavm_ret_index += 1;
                        } else {
                            ep_file.write(format!(
                                "wavm_exported_function_{}({});",
                                aot_generator::cnaming(field),
                                args_with_null.join(", ")
                            ));
                        }
                        ep_file.write("");
                    }
                    _ => panic!("unreachable"),
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
    ep_file.write("}");
    ep_file.close()?;

    gcc::build(&middle)?;

    let mut cmd = std::process::Command::new(middle.path_output.to_str().unwrap());
    let exit_status = cmd.spawn()?.wait()?;

    rog::println!("{:?} {}", middle.path_c, exit_status);
    assert!(exit_status.success());
    Ok(())
}

fn test_single_suit<P: AsRef<std::path::Path>>(
    spec_path: P,
    skip: Vec<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let spec_path = spec_path.as_ref();
    let file_stem = spec_path.file_stem().unwrap().to_str().unwrap();
    let path_json = spec_path.join(format!("{}.json", file_stem));
    let file_json = std::fs::File::open(&path_json).unwrap();
    let json: serde_json::Value = serde_json::from_reader(std::io::BufReader::new(&file_json)).unwrap();

    let mut wasm_file = std::path::PathBuf::new();
    let mut commands: Vec<serde_json::Value> = vec![];

    for command in json["commands"].as_array().unwrap() {
        match command["type"].as_str().unwrap() {
            "module" => {
                if wasm_file.to_str().unwrap() != "" {
                    test_single_test(&wasm_file, commands.clone())?;
                    commands.clear();
                }
                let file_name: &str = command["filename"].as_str().unwrap();
                let nice_name: &str = &file_name.replacen(".", "_", 1);
                if skip.contains(&nice_name) {
                    rog::println!("skip {:?}", nice_name);
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
        test_single_test(&wasm_file, commands.clone())?;
        commands.clear();
    }
    Ok(())
}

#[test]
fn test_posix_x86_64_spec() {
    let wasc_path = std::path::PathBuf::from("./res/posix_x86_64_spectest");
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

    test_single_suit("./res/posix_x86_64_spectest/address", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/align", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/binary", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/binary-leb128", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/br_if", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/br_table", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/break-drop", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/comments", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/const", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/custom", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/data", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/elem", vec!["elem_39.wasm", "elem_40.wasm"]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/endianness", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/f32", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/f32_bitwise", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/f32_cmp", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/f64", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/f64_bitwise", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/f64_cmp", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/float_exprs", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/float_literals", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/float_memory", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/float_misc", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/forward", vec![]).unwrap();
    test_single_suit(
        "./res/posix_x86_64_spectest/func_ptrs",
        vec!["func_ptrs_8.wasm", "func_ptrs_9.wasm"],
    )
    .unwrap();
    test_single_suit("./res/posix_x86_64_spectest/global", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/globals", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/inline-module", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/int_exprs", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/int_literals", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/labels", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/left-to-right", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/load", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/local_get", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/local_set", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/local_tee", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/memory", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/memory_grow", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/memory_redundancy", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/memory_size", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/memory_trap", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/names", vec!["names_3.wasm"]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/nop", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/return", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/select", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/skip-stack-guard-page", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/stack", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/start", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/store", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/switch", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/table", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/token", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/traps", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/type", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/typecheck", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/unreachable", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/unreached-invalid", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/unwind", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/utf8-custom-section-id", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/utf8-import-field", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/utf8-import-module", vec![]).unwrap();
    test_single_suit("./res/posix_x86_64_spectest/utf8-invalid-encoding", vec![]).unwrap();
}
