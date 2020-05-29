use rand::Rng;

#[derive(Clone, Debug, Default)]
struct Config {
    wavm_binary: String,
}

#[derive(Clone, Debug, Default)]
struct Middle {
    config: Config,
    source_file: std::path::PathBuf,
    source_file_stem: String,
    temp_dir: std::path::PathBuf,
    wavm_precompiled_wasm: std::path::PathBuf,
}

fn wasc_create_build_temp_dir(middle: &mut Middle) {
    let temp_dir_root = std::env::temp_dir();
    let random_4_byte = rand::thread_rng().gen::<[u8; 4]>();
    let random_4_byte_hex = hex::encode(random_4_byte);
    let temp_dir_basename = String::from("wasc-") + &random_4_byte_hex;
    let temp_dir = temp_dir_root.join(temp_dir_basename);
    rog::debugln!("wasc_create_build_temp_dir temp_dir={:?}", temp_dir);
    std::fs::create_dir(temp_dir.clone()).unwrap();
    middle.temp_dir = temp_dir;
}

fn wasc_init<P: AsRef<std::path::Path>>(middle: &mut Middle, source: P) {
    let source_path = source.as_ref();
    let source_file_name = source_path.clone().file_name().unwrap();
    let dest_path = middle.temp_dir.clone().join(source_file_name);
    rog::debugln!("wasc_init copy from={:?} to={:?}", source_path, dest_path);
    std::fs::copy(source_path, dest_path.clone()).unwrap();
    middle.source_file = dest_path.clone();
    middle.source_file_stem = dest_path.file_stem().unwrap().to_str().unwrap().into();
}

fn wasc_remove_build_temp_dir(middle: &mut Middle) {
    rog::debugln!("wasc_remove_build_temp_dir temp_dir={:?}", middle.temp_dir);
    std::fs::remove_dir_all(middle.temp_dir.clone()).unwrap();
}

fn wavm_compile(middle: &mut Middle) {
    let outwasm = middle
        .temp_dir
        .join(middle.source_file_stem.clone() + "_precompiled")
        .with_extension("wasm");
    rog::debugln!("wavm_compile outwasm={:?}", outwasm);
    let mut cmd = std::process::Command::new(middle.config.wavm_binary.clone());
    cmd.arg("compile")
        .arg("--enable")
        .arg("all")
        .arg(middle.source_file.clone())
        .arg(outwasm.to_str().unwrap());
    rog::debugln!("wavm_compile command={:?}", cmd);
    cmd.spawn().unwrap().wait().unwrap();
    middle.wavm_precompiled_wasm = outwasm;
}

fn main() {
    rog::reg("wasc");

    let mut source = String::from("");
    {
        let mut ap = argparse::ArgumentParser::new();
        ap.set_description("WASC: WebAssembly native compilter");
        ap.refer(&mut source)
            .add_argument("source", argparse::Store, "WASM/WA(S)T source file");
        ap.parse_args_or_exit();
    }
    let config = Config {
        wavm_binary: String::from("/src/wasc/third_party/WAVM/build/bin/wavm"),
    };
    rog::debugln!("main config={:?}", config);
    let mut middle = Middle::default();
    middle.config = config;

    wasc_create_build_temp_dir(&mut middle);
    wasc_init(&mut middle, source);
    wavm_compile(&mut middle);
    wasc_remove_build_temp_dir(&mut middle);

    // aot("examples/helloworld.wasm", "examples/helloworld");

    // std::fs::write(
    //     "examples/dummy.c",
    //     "#include \"helloworld_glue.h\"\n#include \"../abi/posix_wasi_abi.h\"",
    // );

    // let mut cmd = std::process::Command::new("gcc");
    // cmd.arg("-g")
    //     .arg("-o")
    //     .arg("examples/helloworld")
    //     .arg("examples/helloworld.o")
    //     .arg("examples/dummy.c");
    // cmd.spawn().unwrap().wait().unwrap();
}

#[macro_use]
extern crate log;

use std::env;
use std::fs::File;
use std::io::{self, prelude::*};
use wasmparser::{
    ElemSectionEntryTable, ElementItem, ExternalKind, FuncType, GlobalType, ImportSectionEntryType,
    MemoryType, Operator, Parser, ParserState, ResizableLimits, SectionCode, TableType, Type,
    WasmDecoder,
};

enum CurrentSection {
    Empty,
    Data,
    Global,
    Element,
}

fn aot(source: &str, dest: &str) {
    let buf: Vec<u8> = read_wasm(source).unwrap();
    let mut glue_file = File::create(format!("{}_glue.h", dest)).expect("create glue file");
    let mut object_file = File::create(format!("{}.o", dest)).expect("create object file");
    let header_id = format!("{}_GLUE_H", dest);
    glue_file
        .write_all(
            format!(
                "#include<stddef.h>
#include<stdint.h>

#ifndef {}
#define {}

typedef struct {{
  void* dummy;
  int32_t value;
}} wavm_ret_int32_t;

typedef struct {{
  void* dummy;
  int64_t value;
}} wavm_ret_int64_t;

typedef struct {{
  void* dummy;
  float value;
}} wavm_ret_float;

typedef struct {{
  void* dummy;
  double value;
}} wavm_ret_double;

const uint64_t functionDefMutableData = 0;
const uint64_t biasedInstanceId = 0;
\n",
                header_id, header_id
            )
            .as_bytes(),
        )
        .expect("write glue file");

    let mut parser = Parser::new(&buf);
    let mut section_name: Option<String> = None;
    let mut type_entries: Vec<FuncType> = vec![];
    let mut next_import_index = 0;
    let mut next_function_index = 0;
    let mut function_entries: Vec<Option<usize>> = vec![];
    let mut has_main = false;
    let mut memories: Vec<Vec<u8>> = vec![];
    let mut data_index: Option<usize> = None;
    let mut data_offset: Option<usize> = None;
    let mut current_section = CurrentSection::Empty;
    let mut next_global_index = 0;
    let mut global_content_type = Type::EmptyBlockType;
    let mut global_mutable = false;
    let mut tables: Vec<Vec<String>> = vec![];
    let mut table_index: Option<usize> = None;
    let mut table_offset: Option<usize> = None;
    loop {
        let state = parser.read();
        match *state {
            ParserState::BeginSection { code, .. } => {
                if let SectionCode::Custom { name, .. } = code {
                    section_name = Some(name.to_string());
                }
            }
            ParserState::EndSection => {
                section_name = None;
            }
            ParserState::SectionRawData(data) => {
                if section_name.clone().unwrap_or("".to_string()) == "wavm.precompiled_object" {
                    object_file.write_all(data).expect("write object file");
                }
            }
            ParserState::TypeSectionEntry(ref t) => {
                glue_file
                    .write_all(
                        format!("const uint64_t typeId{} = 0;\n", type_entries.len()).as_bytes(),
                    )
                    .expect("write glue file");
                type_entries.push(t.clone());
            }
            ParserState::ImportSectionEntry {
                module,
                field,
                ty: ImportSectionEntryType::Function(index),
            } => {
                function_entries.push(None);
                let func_type = &type_entries[index as usize];
                let name = format!("wavm_{}_{}", module, field);
                let import_symbol = format!("functionImport{}", next_import_index);
                glue_file
                    .write_all(format!("#define {} {}\n", name, import_symbol).as_bytes())
                    .expect("write glue file");
                next_import_index += 1;
                glue_file
                    .write_all(
                        format!(
                            "extern {};\n",
                            convert_func_type_to_c_function(&func_type, import_symbol)
                        )
                        .as_bytes(),
                    )
                    .expect("write glue file");
            }
            ParserState::FunctionSectionEntry(type_entry_index) => {
                let func_type = &type_entries[type_entry_index as usize];
                let name = format!("functionDef{}", next_function_index);
                glue_file
                    .write_all(
                        format!(
                            "extern {};
const uint64_t functionDefMutableDatas{} = 0;\n",
                            convert_func_type_to_c_function(&func_type, name),
                            next_function_index,
                        )
                        .as_bytes(),
                    )
                    .expect("write glue file");
                function_entries.push(Some(next_function_index));
                next_function_index += 1;
            }
            ParserState::ExportSectionEntry {
                field,
                kind: ExternalKind::Function,
                index,
            } => {
                let function_index =
                    function_entries[index as usize].expect("Exported function should exist!");
                glue_file
                    .write_all(
                        format!(
                            "#define wavm_exported_function_{} functionDef{}\n",
                            field, function_index,
                        )
                        .as_bytes(),
                    )
                    .expect("write glue file");

                if field == "_start" {
                    has_main = true;
                }
            }
            ParserState::TableSectionEntry(TableType {
                element_type: Type::AnyFunc,
                limits: ResizableLimits { initial: count, .. },
            }) => {
                let mut table = vec![];
                table.resize(count as usize, "0".to_string());
                tables.push(table);
            }
            ParserState::MemorySectionEntry(MemoryType {
                limits: ResizableLimits { initial: pages, .. },
                ..
            }) => {
                let mut mem = vec![];
                mem.resize(pages as usize * 64 * 1024, 0);
                memories.push(mem);
            }
            ParserState::BeginActiveDataSectionEntry(i) => {
                data_index = Some(i as usize);
                current_section = CurrentSection::Data;
            }
            ParserState::EndDataSectionEntry => {
                data_index = None;
                data_offset = None;
                current_section = CurrentSection::Empty;
            }
            ParserState::InitExpressionOperator(ref value) => match current_section {
                CurrentSection::Data => {
                    if let Operator::I32Const { value } = value {
                        data_offset = Some(*value as usize);
                    }
                }
                CurrentSection::Element => {
                    if let Operator::I32Const { value } = value {
                        table_offset = Some(*value as usize);
                    }
                }
                CurrentSection::Global => {
                    glue_file
                        .write_all(
                            generate_global_entry(
                                next_global_index,
                                &global_content_type,
                                global_mutable,
                                &value,
                            )
                            .as_bytes(),
                        )
                        .expect("write glue file!");
                    next_global_index += 1;
                }
                CurrentSection::Empty => {
                    debug!("Omitted init expression: {:?}", value);
                }
            },
            ParserState::DataSectionEntryBodyChunk(data) => {
                let index = data_index.unwrap();
                let offset = data_offset.unwrap();
                memories[index][offset..offset + data.len()].copy_from_slice(&data);
                data_offset = Some(offset + data.len());
            }
            ParserState::ElementSectionEntryBody(ref items) => {
                let index = table_index.unwrap();
                let offset = table_offset.unwrap();

                for (i, item) in items.iter().enumerate() {
                    if let ElementItem::Func(func_index) = item {
                        tables[index][offset + i] =
                            format!("((uintptr_t) (functionDef{}))", func_index);
                    }
                }
            }
            ParserState::BeginGlobalSectionEntry(GlobalType {
                content_type,
                mutable,
            }) => {
                global_content_type = content_type;
                global_mutable = mutable;
                current_section = CurrentSection::Global;
            }
            ParserState::EndGlobalSectionEntry => {
                current_section = CurrentSection::Empty;
            }
            ParserState::BeginElementSectionEntry {
                table: ElemSectionEntryTable::Active(i),
                ty: Type::AnyFunc,
            } => {
                table_index = Some(i as usize);
                current_section = CurrentSection::Element;
            }
            ParserState::EndElementSectionEntry => {
                table_index = None;
                table_offset = None;
                current_section = CurrentSection::Empty;
            }
            ParserState::EndWasm => break,
            ParserState::Error(ref err) => panic!("Error: {:?}", err),
            _ => debug!("Unprocessed states: {:?}", state),
        }
    }

    for (i, table) in tables.iter().enumerate() {
        glue_file
            .write_all(format!("uint32_t table{}_length = {};\n", i, table.len()).as_bytes())
            .expect("write glue file");
        glue_file
            .write_all(format!("uintptr_t table{}[{}] = {{", i, table.len()).as_bytes())
            .expect("write glue file");
        let reversed_striped_table: Vec<String> = table
            .iter()
            .rev()
            .map(|x| x.clone())
            .skip_while(|c| *c == "0")
            .collect();
        let mut striped_table: Vec<String> = reversed_striped_table.into_iter().rev().collect();
        if striped_table.len() == 0 {
            striped_table.push("0".to_string());
        }
        for (j, c) in striped_table.iter().enumerate() {
            if j % 4 == 0 {
                glue_file.write_all(b"\n  ").expect("write glue file");
            }
            glue_file.write_all(c.as_bytes()).expect("write glue file");
            if j < striped_table.len() - 1 {
                glue_file.write_all(b", ").expect("write glue file");
            }
        }
        glue_file.write_all(b"\n};\n").expect("write glue file");
        glue_file
            .write_all(
                format!(
                    "uintptr_t* tableOffset{} = table{};
#define TABLE{}_DEFINED 1\n",
                    i, i, i
                )
                .as_bytes(),
            )
            .expect("write glue file");
    }

    for (i, mem) in memories.iter().enumerate() {
        glue_file
            .write_all(format!("uint32_t memory{}_length = {};\n", i, mem.len()).as_bytes())
            .expect("write glue file");
        glue_file
            .write_all(
                format!(
                    "uint8_t __attribute__((section (\".wasm_memory\"))) memory{}[{}] = {{",
                    i,
                    mem.len()
                )
                .as_bytes(),
            )
            .expect("write glue file");
        let reversed_striped_mem: Vec<u8> = mem
            .iter()
            .rev()
            .map(|x| *x)
            .skip_while(|c| *c == 0)
            .collect();
        let mut striped_mem: Vec<u8> = reversed_striped_mem.into_iter().rev().collect();
        if striped_mem.len() == 0 {
            striped_mem.push(0);
        }
        for (j, c) in striped_mem.iter().enumerate() {
            if j % 32 == 0 {
                glue_file.write_all(b"\n  ").expect("write glue file");
            }
            glue_file
                .write_all(format!("0x{:x}", c).as_bytes())
                .expect("write glue file");
            if j < striped_mem.len() - 1 {
                glue_file.write_all(b", ").expect("write glue file");
            }
        }
        glue_file.write_all(b"\n};\n").expect("write glue file");
        glue_file
            .write_all(
                format!(
                    "uint8_t* memoryOffset{} = memory{};
#define MEMORY{}_DEFINED 1\n",
                    i, i, i
                )
                .as_bytes(),
            )
            .expect("write glue file");
    }

    if has_main {
        glue_file
            .write_all(
                b"\nint main() {
  wavm_exported_function__start(NULL);
  // This should not be reached
  return -1;
}\n",
            )
            .expect("write glue file");
    }

    glue_file
        .write_all(format!("\n#endif /* {} */\n", header_id).as_bytes())
        .expect("write glue file");
}

fn wasm_type_to_c_type(t: Type) -> String {
    match t {
        Type::I32 => "int32_t".to_string(),
        Type::I64 => "int64_t".to_string(),
        Type::F32 => "float".to_string(),
        Type::F64 => "double".to_string(),
        _ => panic!("Unsupported type: {:?}", t),
    }
}

fn convert_func_type_to_c_function(func_type: &FuncType, name: String) -> String {
    if func_type.form != Type::Func || func_type.returns.len() > 1 {
        panic!("Invalid func type: {:?}", func_type);
    }
    let mut fields: Vec<String> = func_type
        .params
        .iter()
        .map(|t| wasm_type_to_c_type(*t))
        .collect();
    fields.insert(0, "void*".to_string());
    let return_type = if func_type.returns.len() > 0 {
        format!("wavm_ret_{}", wasm_type_to_c_type(func_type.returns[0]))
    } else {
        "void*".to_string()
    };
    format!("{} ({}) ({})", return_type, name, fields.join(", ")).to_string()
}

fn read_wasm(file: &str) -> io::Result<Vec<u8>> {
    let mut data = Vec::new();
    let mut f = File::open(file)?;
    f.read_to_end(&mut data)?;
    Ok(data)
}

fn generate_global_entry(
    index: usize,
    content_type: &Type,
    mutable: bool,
    value: &Operator,
) -> String {
    let mutable_string = if mutable { "" } else { "const " };
    let type_string = wasm_type_to_c_type(content_type.clone());

    let value_string = match content_type {
        Type::I32 => {
            if let Operator::I32Const { value } = value {
                value.to_string()
            } else {
                panic!("Invalid global value {:?} for type {:?}",)
            }
        }
        Type::I64 => {
            if let Operator::I64Const { value } = value {
                value.to_string()
            } else {
                panic!("Invalid global value {:?} for type {:?}",)
            }
        }
        _ => panic!("Invalid content type: {:?} for global entry", content_type),
    };

    format!(
        "{}{} global{} = {};\n",
        mutable_string, type_string, index, value_string
    )
}
