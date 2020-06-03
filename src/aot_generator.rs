use super::context;
use std::io::Write;
use wasmparser::WasmDecoder;

enum CurrentSection {
    Empty,
    Data,
    Global,
    Element,
}

pub fn glue(middle: &mut context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    let wasm_data: Vec<u8> = std::fs::read(middle.wavm_precompiled_wasm.to_str().unwrap())?;
    rog::debugln!("glue wasm_data.length={:?}", wasm_data.len());
    let file_stem = middle.file_stem.clone();
    let glue_path = middle.prog_dir.join(file_stem.clone() + "_glue.h");
    let object_path = middle.prog_dir.join(file_stem.clone() + ".o");
    rog::debugln!("glue glue_path={:?}", glue_path);
    rog::debugln!("glue object_path={:?}", object_path);
    let mut glue_file = std::fs::File::create(glue_path.clone())?;
    let mut object_file = std::fs::File::create(object_path.clone())?;

    let header_id = format!("{}_GLUE_H", file_stem);
    glue_file.write_all(
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
    )?;

    let mut parser = wasmparser::Parser::new(&wasm_data);
    let mut section_name: Option<String> = None;
    let mut type_entries: Vec<wasmparser::FuncType> = vec![];
    let mut next_import_index = 0;
    let mut next_function_index = 0;
    let mut function_entries: Vec<Option<usize>> = vec![];
    let mut has_main = false;
    let mut memories: Vec<Vec<u8>> = vec![];
    let mut data_index: Option<usize> = None;
    let mut data_offset: Option<usize> = None;
    let mut current_section = CurrentSection::Empty;
    let mut next_global_index = 0;
    let mut global_content_type = wasmparser::Type::EmptyBlockType;
    let mut global_mutable = false;
    let mut tables: Vec<Vec<String>> = vec![];
    let mut table_index: Option<usize> = None;
    let mut table_offset: Option<usize> = None;
    loop {
        let state = parser.read();
        match *state {
            wasmparser::ParserState::BeginSection { code, .. } => {
                if let wasmparser::SectionCode::Custom { name, .. } = code {
                    section_name = Some(name.to_string());
                }
            }
            wasmparser::ParserState::EndSection => {
                section_name = None;
            }
            wasmparser::ParserState::SectionRawData(data) => {
                if section_name.clone().unwrap_or("".to_string()) == "wavm.precompiled_object" {
                    object_file.write_all(data).expect("write object file");
                }
            }
            wasmparser::ParserState::TypeSectionEntry(ref t) => {
                glue_file.write_all(
                    format!("const uint64_t typeId{} = 0;\n", type_entries.len()).as_bytes(),
                )?;
                type_entries.push(t.clone());
            }
            wasmparser::ParserState::ImportSectionEntry {
                module,
                field,
                ty: wasmparser::ImportSectionEntryType::Function(index),
            } => {
                function_entries.push(None);
                let func_type = &type_entries[index as usize];
                let name = format!("wavm_{}_{}", module, field);
                let import_symbol = format!("functionImport{}", next_import_index);
                glue_file.write_all(format!("#define {} {}\n", name, import_symbol).as_bytes())?;
                next_import_index += 1;
                glue_file.write_all(
                    format!(
                        "extern {};\n",
                        convert_func_type_to_c_function(&func_type, import_symbol)
                    )
                    .as_bytes(),
                )?;
            }
            wasmparser::ParserState::FunctionSectionEntry(type_entry_index) => {
                let func_type = &type_entries[type_entry_index as usize];
                let name = format!("functionDef{}", next_function_index);
                glue_file.write_all(
                    format!(
                        "extern {};
const uint64_t functionDefMutableDatas{} = 0;\n",
                        convert_func_type_to_c_function(&func_type, name),
                        next_function_index,
                    )
                    .as_bytes(),
                )?;
                function_entries.push(Some(next_function_index));
                next_function_index += 1;
            }
            wasmparser::ParserState::ExportSectionEntry {
                field,
                kind: wasmparser::ExternalKind::Function,
                index,
            } => {
                let function_index =
                    function_entries[index as usize].expect("Exported function should exist!");
                glue_file.write_all(
                    format!(
                        "#define wavm_exported_function_{} functionDef{}\n",
                        convert_func_name_to_c_function(field),
                        function_index,
                    )
                    .as_bytes(),
                )?;

                if field == "_start" {
                    has_main = true;
                }
            }
            wasmparser::ParserState::TableSectionEntry(wasmparser::TableType {
                element_type: wasmparser::Type::AnyFunc,
                limits: wasmparser::ResizableLimits { initial: count, .. },
            }) => {
                let mut table = vec![];
                table.resize(count as usize, "0".to_string());
                tables.push(table);
            }
            wasmparser::ParserState::MemorySectionEntry(wasmparser::MemoryType {
                limits: wasmparser::ResizableLimits { initial: pages, .. },
                ..
            }) => {
                let mut mem = vec![];
                mem.resize(pages as usize * 64 * 1024, 0);
                memories.push(mem);
            }
            wasmparser::ParserState::BeginActiveDataSectionEntry(i) => {
                data_index = Some(i as usize);
                current_section = CurrentSection::Data;
            }
            wasmparser::ParserState::EndDataSectionEntry => {
                data_index = None;
                data_offset = None;
                current_section = CurrentSection::Empty;
            }
            wasmparser::ParserState::InitExpressionOperator(ref value) => match current_section {
                CurrentSection::Data => {
                    if let wasmparser::Operator::I32Const { value } = value {
                        data_offset = Some(*value as usize);
                    }
                }
                CurrentSection::Element => {
                    if let wasmparser::Operator::I32Const { value } = value {
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
                    rog::debugln!("Omitted init expression: {:?}", value);
                }
            },
            wasmparser::ParserState::DataSectionEntryBodyChunk(data) => {
                let index = data_index.unwrap();
                let offset = data_offset.unwrap();
                memories[index][offset..offset + data.len()].copy_from_slice(&data);
                data_offset = Some(offset + data.len());
            }
            wasmparser::ParserState::ElementSectionEntryBody(ref items) => {
                let index = table_index.unwrap();
                let offset = table_offset.unwrap();

                for (i, item) in items.iter().enumerate() {
                    if let wasmparser::ElementItem::Func(func_index) = item {
                        tables[index][offset + i] =
                            format!("((uintptr_t) (functionDef{}))", func_index);
                    }
                }
            }
            wasmparser::ParserState::BeginGlobalSectionEntry(wasmparser::GlobalType {
                content_type,
                mutable,
            }) => {
                global_content_type = content_type;
                global_mutable = mutable;
                current_section = CurrentSection::Global;
            }
            wasmparser::ParserState::EndGlobalSectionEntry => {
                current_section = CurrentSection::Empty;
            }
            wasmparser::ParserState::BeginElementSectionEntry {
                table: wasmparser::ElemSectionEntryTable::Active(i),
                ty: wasmparser::Type::AnyFunc,
            } => {
                table_index = Some(i as usize);
                current_section = CurrentSection::Element;
            }
            wasmparser::ParserState::EndElementSectionEntry => {
                table_index = None;
                table_offset = None;
                current_section = CurrentSection::Empty;
            }
            wasmparser::ParserState::EndWasm => break,
            wasmparser::ParserState::Error(ref err) => panic!("Error: {:?}", err),
            _ => rog::debugln!("Unprocessed states: {:?}", state),
        }
    }

    for (i, table) in tables.iter().enumerate() {
        glue_file
            .write_all(format!("uint32_t table{}_length = {};\n", i, table.len()).as_bytes())?;
        glue_file.write_all(format!("uintptr_t table{}[{}] = {{", i, table.len()).as_bytes())?;
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
                glue_file.write_all(b"\n  ")?;
            }
            glue_file.write_all(c.as_bytes())?;
            if j < striped_table.len() - 1 {
                glue_file.write_all(b", ")?;
            }
        }
        glue_file.write_all(b"\n};\n")?;
        glue_file.write_all(
            format!(
                "uintptr_t* tableOffset{} = table{};
#define TABLE{}_DEFINED 1\n",
                i, i, i
            )
            .as_bytes(),
        )?;
    }

    for (i, mem) in memories.iter().enumerate() {
        glue_file
            .write_all(format!("uint32_t memory{}_length = {};\n", i, mem.len()).as_bytes())?;
        glue_file.write_all(
            format!(
                "uint8_t __attribute__((section (\".wasm_memory\"))) memory{}[{}] = {{",
                i,
                mem.len()
            )
            .as_bytes(),
        )?;
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
                glue_file.write_all(b"\n  ")?;
            }
            glue_file.write_all(format!("0x{:x}", c).as_bytes())?;
            if j < striped_mem.len() - 1 {
                glue_file.write_all(b", ")?;
            }
        }
        glue_file.write_all(b"\n};\n")?;
        glue_file.write_all(
            format!(
                "uint8_t* memoryOffset{} = memory{};
#define MEMORY{}_DEFINED 1\n",
                i, i, i
            )
            .as_bytes(),
        )?;
    }

    if has_main {
        glue_file.write_all(
            b"\nint main() {
  wavm_exported_function__start(NULL);
  // This should not be reached
  return -1;
}\n",
        )?;
    }

    glue_file.write_all(format!("\n#endif /* {} */\n", header_id).as_bytes())?;

    middle.aot_object = object_path;
    middle.aot_glue = glue_path;
    Ok(())
}

fn wasm_type_to_c_type(t: wasmparser::Type) -> String {
    match t {
        wasmparser::Type::I32 => "int32_t".to_string(),
        wasmparser::Type::I64 => "int64_t".to_string(),
        wasmparser::Type::F32 => "float".to_string(),
        wasmparser::Type::F64 => "double".to_string(),
        _ => panic!("Unsupported type: {:?}", t),
    }
}

pub fn convert_func_name_to_c_function(name: &str) -> String {
    name.replace("-", "_").replace(".", "_")
}

fn convert_func_type_to_c_function(func_type: &wasmparser::FuncType, name: String) -> String {
    if func_type.form != wasmparser::Type::Func || func_type.returns.len() > 1 {
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

fn generate_global_entry(
    index: usize,
    content_type: &wasmparser::Type,
    mutable: bool,
    value: &wasmparser::Operator,
) -> String {
    let mutable_string = if mutable { "" } else { "const " };
    let type_string = wasm_type_to_c_type(content_type.clone());

    let value_string = match content_type {
        wasmparser::Type::I32 => {
            if let wasmparser::Operator::I32Const { value } = value {
                value.to_string()
            } else {
                panic!("Invalid global value {:?} for type {:?}",)
            }
        }
        wasmparser::Type::I64 => {
            if let wasmparser::Operator::I64Const { value } = value {
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
