use super::context;
use std::io::Write;
use wasmparser::WasmDecoder;

// See: https://webassembly.github.io/spec/core/valid/instructions.html#constant-expressions.
#[derive(Debug)]
pub enum ConstantOperator {
    I32Const { value: i32 },
    I64Const { value: i64 },
    F32Const { value: u32 },
    F64Const { value: u64 },
    GlobalGet { global_index: u32 },
}

impl<'a> From<wasmparser::Operator<'a>> for ConstantOperator {
    fn from(o: wasmparser::Operator) -> Self {
        match o {
            wasmparser::Operator::I32Const { value } => ConstantOperator::I32Const { value: value },
            wasmparser::Operator::I64Const { value } => ConstantOperator::I64Const { value: value },
            wasmparser::Operator::F32Const { value } => ConstantOperator::F32Const { value: value.bits() },
            wasmparser::Operator::F64Const { value } => ConstantOperator::F64Const { value: value.bits() },
            wasmparser::Operator::GlobalGet { global_index } => ConstantOperator::GlobalGet {
                global_index: global_index,
            },
            _ => unimplemented!(),
        }
    }
}

// Custom sections have the id 0. They are intended to be used for debugging
// information or third-party extensions, and are ignored by the WebAssembly
// semantics. Their contents consist of a name further identifying the custom
// section, followed by an uninterpreted sequence of bytes for custom use.
#[derive(Debug)]
struct Custom {
    name: String,
    data: Vec<u8>,
}

// The imports component of a module defines a set of imports that are required for instantiation.
#[derive(Debug)]
struct Import {
    module: String,
    field: String,
    ty: wasmparser::ImportSectionEntryType,
}

// The globals component of a module defines a vector of global variables.
#[derive(Debug)]
struct Global {
    global_type: wasmparser::GlobalType,
    expr: Option<ConstantOperator>,
}

// The initial contents of a memory are zero-valued bytes. The data component of a module defines a vector of data
// segments that initialize a range of memory, at a given offset, with a static vector of bytes.
#[derive(Debug)]
struct Data {
    memory_index: u32,
    offset: Option<ConstantOperator>,
    init: Vec<u8>,
}

// WebAssembly module definition.
#[derive(Debug, Default)]
struct Module {
    custom_list: Vec<Custom>,
    type_list: Vec<wasmparser::FuncType>,
    function_list: Vec<u32>,
    table_list: Vec<wasmparser::TableType>,
    memory_list: Vec<wasmparser::MemoryType>,
    global_list: Vec<Global>,
    element_list: Vec<u8>,
    data_list: Vec<Data>,
    start: Option<u32>,
    import_list: Vec<Import>,
    export_list: Vec<u8>,
}

impl Module {
    // Build the module from raw bytes.
    fn from(wasm: Vec<u8>) -> Self {
        let mut wasm_module: Module = Module::default();
        let mut parser = wasmparser::Parser::new(&wasm);
        let mut section_code: Option<wasmparser::SectionCode> = None;
        while !parser.eof() {
            let state = parser.read();
            match *state {
                wasmparser::ParserState::StartSectionEntry(function_index) => {
                    wasm_module.start = Some(function_index);
                }
                wasmparser::ParserState::BeginSection { code, .. } => {
                    section_code = Some(code);
                }
                wasmparser::ParserState::EndSection => {
                    section_code = None;
                }
                wasmparser::ParserState::SectionRawData(data) => {
                    if let Some(wasmparser::SectionCode::Custom { name, .. }) = section_code {
                        let custom = Custom {
                            name: name.to_string(),
                            data: data.to_vec(),
                        };
                        wasm_module.custom_list.push(custom);
                    }
                }
                wasmparser::ParserState::TypeSectionEntry(ref func_type) => {
                    wasm_module.type_list.push(func_type.clone());
                }
                wasmparser::ParserState::FunctionSectionEntry(func_type_index) => {
                    wasm_module.function_list.push(func_type_index);
                }
                wasmparser::ParserState::TableSectionEntry(table_type) => {
                    wasm_module.table_list.push(table_type);
                }
                wasmparser::ParserState::MemorySectionEntry(memory_type) => {
                    wasm_module.memory_list.push(memory_type);
                }
                wasmparser::ParserState::BeginGlobalSectionEntry(global_type) => {
                    let global = Global {
                        global_type: global_type,
                        expr: None,
                    };
                    wasm_module.global_list.push(global);
                }
                wasmparser::ParserState::EndGlobalSectionEntry => {}
                wasmparser::ParserState::InitExpressionOperator(ref value) => match section_code {
                    Some(wasmparser::SectionCode::Global) => {
                        wasm_module.global_list.last_mut().unwrap().expr = Some(value.clone().into())
                    }
                    Some(wasmparser::SectionCode::Data) => {
                        wasm_module.data_list.last_mut().unwrap().offset = Some(value.clone().into())
                    }
                    _ => {}
                },
                wasmparser::ParserState::BeginActiveDataSectionEntry(memory_index) => {
                    let data = Data {
                        memory_index: memory_index,
                        offset: None,
                        init: vec![],
                    };
                    wasm_module.data_list.push(data);
                }
                wasmparser::ParserState::EndDataSectionEntry => {}
                wasmparser::ParserState::DataSectionEntryBodyChunk(init) => {
                    wasm_module.data_list.last_mut().unwrap().init = init.to_vec();
                }
                wasmparser::ParserState::ImportSectionEntry { module, field, ty } => {
                    wasm_module.import_list.push(Import {
                        module: module.to_string(),
                        field: field.to_string(),
                        ty: ty,
                    });
                }
                wasmparser::ParserState::Error(ref err) => panic!("Error: {:?}", err),
                _ => {}
            }
        }
        wasm_module
    }
}

// Functions that map between the symbols used for externally visible functions and the function.
fn get_external_name(base_name: &str, index: u32) -> String {
    format!("{}{}", base_name, index)
}

enum CurrentSection {
    Empty,
    Element,
}

enum GlobalValue {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
    Imported(String),
}

impl GlobalValue {
    fn as_i32(&self) -> i32 {
        if let GlobalValue::I32(x) = self {
            return *x;
        }
        panic!("unreachable")
    }
}

#[derive(Debug)]
struct DynamicMemory {
    index: usize,
    offset: String,
    data: Vec<u8>,
}

#[derive(Debug)]
struct DynamicTableEntry {
    index: usize,
    offset: String,
    shift: usize,
    func_index: usize,
}

pub fn generate(middle: &mut context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    let wasm_data: Vec<u8> = std::fs::read(middle.wavm_precompiled_wasm.to_str().unwrap())?;
    let wasm_module = Module::from(wasm_data.clone());

    let file_stem = middle.file_stem.clone();
    let glue_path = middle.prog_dir.join(file_stem.clone() + "_glue.h");
    let object_path = middle.prog_dir.join(file_stem.clone() + ".o");
    let mut glue_file = std::fs::File::create(glue_path.clone())?;
    let mut object_file = std::fs::File::create(object_path.clone())?;

    for e in wasm_module.custom_list {
        if e.name == "wavm.precompiled_object" {
            object_file.write_all(&e.data)?;
        }
    }

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

typedef struct memory_runtime_data {{
    uint8_t* base;
    uint64_t num_pages;
}} memory_runtime_data;

const uint64_t functionDefMutableData = 0;
const uint64_t biasedInstanceId = 0;
const uint64_t tableReferenceBias = 0;
\n",
            header_id, header_id
        )
        .as_bytes(),
    )?;

    let mut parser = wasmparser::Parser::new(&wasm_data);
    let mut next_import_index = 0;
    let mut next_import_global_index = 0;
    let mut next_function_index = 0;
    let mut function_entries: Vec<Option<usize>> = vec![];
    let mut function_names: Vec<String> = vec![];
    let mut has_main = false;
    let mut memories: Vec<Vec<u8>> = vec![];
    let mut max_page_num: Option<u32> = None;
    let mut dynamic_memories: Vec<DynamicMemory> = vec![];
    let mut current_section = CurrentSection::Empty;
    let mut next_global_index = 0;
    let mut global_values: Vec<GlobalValue> = vec![];
    let mut tables: Vec<Vec<String>> = vec![];
    let mut table_index: Option<usize> = None;
    let mut table_offset: Option<usize> = None;
    let mut dynamic_table_offset: Option<String> = None;
    let mut dynamic_tables: Vec<DynamicTableEntry> = vec![];

    for (i, _) in wasm_module.type_list.iter().enumerate() {
        glue_file.write_all(format!("const uint64_t {} = 0;\n", get_external_name("typeId", i as u32)).as_bytes())?;
    }
    for e in wasm_module.global_list {
        glue_file.write_all(
            generate_global_entry(
                next_global_index,
                &e.global_type.content_type,
                e.global_type.mutable,
                &e.expr.unwrap(),
                &mut global_values,
            )
            .as_bytes(),
        )?;
        next_global_index += 1;
    }
    for e in wasm_module.import_list {
        match e.ty {
            wasmparser::ImportSectionEntryType::Function(func_type_index) => {
                function_entries.push(None);
                let func_type = &wasm_module.type_list[func_type_index as usize];
                let name = format!("wavm_{}_{}", e.module, e.field);
                let import_symbol = get_external_name("functionImport", next_import_index);
                glue_file.write_all(format!("#define {} {}\n", name, import_symbol).as_bytes())?;
                next_import_index += 1;
                glue_file.write_all(
                    format!(
                        "extern {};\n",
                        convert_func_type_to_c_function(&func_type, import_symbol.clone())
                    )
                    .as_bytes(),
                )?;
                function_names.push(name);
            }
            wasmparser::ImportSectionEntryType::Table(wasmparser::TableType {
                element_type: wasmparser::Type::AnyFunc,
                limits: wasmparser::ResizableLimits { initial, .. },
            }) => {
                let mut table = vec![];
                table.resize(std::cmp::max(256, initial as usize), "0".to_string()); // TODO: implement import table.
                tables.push(table);
            }
            wasmparser::ImportSectionEntryType::Memory(wasmparser::MemoryType {
                limits: wasmparser::ResizableLimits { initial, .. },
                ..
            }) => {
                let mut mem = vec![];
                mem.resize(std::cmp::max(1, initial as usize) * 64 * 1024, 0);
                memories.push(mem);
            }
            wasmparser::ImportSectionEntryType::Global(wasmparser::GlobalType { content_type, .. }) => {
                // #define wavm_spectest_global_i32 global0
                // extern int32_t global0;
                let name = format!("wavm_{}_{}", e.module, e.field);
                let import_symbol = get_external_name("global", next_import_global_index);
                let global_type = wasm_type_to_c_type(content_type);
                glue_file.write_all(format!("#define {} {}\n", name, import_symbol).as_bytes())?;
                glue_file.write_all(format!("extern {} {};\n", global_type, import_symbol).as_bytes())?;
                global_values.push(GlobalValue::Imported(name.clone()));
                next_import_global_index += 1;
            }
            _ => {}
        }
    }
    for e in wasm_module.function_list {
        let func_type = &wasm_module.type_list[e as usize];
        let name = get_external_name("functionDef", next_function_index);
        glue_file.write_all(
            format!(
                "extern {};
const uint64_t {} = 0;\n",
                convert_func_type_to_c_function(&func_type, name.clone()),
                get_external_name("functionDefMutableDatas", next_function_index),
            )
            .as_bytes(),
        )?;
        function_entries.push(Some(next_function_index as usize));
        next_function_index += 1;
        function_names.push(name.clone());
    }
    for e in wasm_module.memory_list {
        let mut mem = vec![];
        mem.resize(e.limits.initial as usize * 64 * 1024, 0);
        memories.push(mem);
        max_page_num = e.limits.maximum;
    }
    for e in wasm_module.table_list {
        let mut table = vec![];
        table.resize(e.limits.initial as usize, "0".to_string());
        tables.push(table);
    }
    for e in wasm_module.data_list {
        match e.offset {
            Some(ConstantOperator::I32Const { value }) => {
                let offset = value as usize;
                memories[e.memory_index as usize][offset..offset + e.init.len()].copy_from_slice(&e.init);
            }
            Some(ConstantOperator::GlobalGet { global_index }) => {
                if let GlobalValue::Imported(s) = &global_values[global_index as usize] {
                    let dmemory = DynamicMemory {
                        index: global_index as usize,
                        offset: s.to_string(),
                        data: e.init.to_vec(),
                    };
                    dynamic_memories.push(dmemory);
                }
            }
            _ => {}
        }
    }

    loop {
        let state = parser.read();
        match *state {
            wasmparser::ParserState::ExportSectionEntry {
                field,
                kind: wasmparser::ExternalKind::Function,
                index,
            } => {
                let function_index = function_entries[index as usize].expect("Exported function should exist!");
                glue_file.write_all(
                    format!(
                        "#define wavm_exported_function_{} {}\n",
                        convert_func_name_to_c_function(field),
                        get_external_name("functionDef", function_index as u32),
                    )
                    .as_bytes(),
                )?;

                if field == "_start" {
                    has_main = true;
                }
            }
            wasmparser::ParserState::InitExpressionOperator(ref value) => match current_section {
                CurrentSection::Element => {
                    if let wasmparser::Operator::I32Const { value } = value {
                        table_offset = Some(*value as usize);
                    }
                    if let wasmparser::Operator::GlobalGet { global_index } = value {
                        let global_value = &global_values[*global_index as usize];
                        if let GlobalValue::Imported(x) = global_value {
                            dynamic_table_offset = Some(x.to_string())
                        } else {
                            table_offset = Some(global_value.as_i32() as usize)
                        }
                    }
                }
                CurrentSection::Empty => {
                    rog::debugln!("Omitted init expression: {:?}", value);
                }
            },
            wasmparser::ParserState::ElementSectionEntryBody(ref items) => {
                let index = table_index.unwrap();
                if let Some(x) = dynamic_table_offset.clone() {
                    for (i, item) in items.iter().enumerate() {
                        if let wasmparser::ElementItem::Func(func_index) = item {
                            dynamic_tables.push(DynamicTableEntry {
                                index: index,
                                offset: x.clone(),
                                shift: i,
                                func_index: *func_index as usize,
                            });
                        }
                    }
                } else {
                    let offset = table_offset.unwrap();
                    for (i, item) in items.iter().enumerate() {
                        if let wasmparser::ElementItem::Func(func_index) = item {
                            tables[index][offset + i] =
                                format!("((uintptr_t) ({}))", get_external_name("functionDef", *func_index));
                        }
                    }
                }
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
                dynamic_table_offset = None;
                current_section = CurrentSection::Empty;
            }
            wasmparser::ParserState::EndWasm => break,
            wasmparser::ParserState::Error(ref err) => panic!("Error: {:?}", err),
            _ => rog::debugln!("Unprocessed states: {:?}", state),
        }
    }

    for (i, table) in tables.iter().enumerate() {
        glue_file.write_all(format!("uint32_t table{}_length = {};\n", i, table.len()).as_bytes())?;
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
                "uintptr_t* {} = table{};
#define TABLE{}_DEFINED 1\n",
                get_external_name("tableOffset", i as u32),
                i,
                i
            )
            .as_bytes(),
        )?;
    }

    for (i, mem) in memories.iter().enumerate() {
        glue_file.write_all(format!("uint32_t memory{}_length = {};\n", i, mem.len()).as_bytes())?;
        glue_file.write_all(
            format!(
                "uint8_t __attribute__((section (\".wasm_memory\"))) memory{}[{}] = {{",
                i,
                mem.len()
            )
            .as_bytes(),
        )?;
        let reversed_striped_mem: Vec<u8> = mem.iter().rev().map(|x| *x).skip_while(|c| *c == 0).collect();
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
                "struct memory_runtime_data {} = {{memory{}, {}}};
#define MEMORY{}_DEFINED 1\n",
                get_external_name("memoryOffset", i as u32),
                i,
                mem.len() / 65536,
                i
            )
            .as_bytes(),
        )?;
        if let Some(x) = max_page_num {
            glue_file.write_all(format!("#define WAVM_MAX_PAGE {}\n", x.to_string()).as_bytes())?;
        }
    }

    for (i, mem) in dynamic_memories.iter().enumerate() {
        glue_file.write_all(format!("uint32_t data{}_length = {};\n", i, mem.data.len()).as_bytes())?;
        glue_file.write_all(format!("uint8_t data{}[{}] = {{", i, mem.data.len()).as_bytes())?;
        for (j, c) in mem.data.iter().enumerate() {
            if j % 32 == 0 {
                glue_file.write_all(b"\n  ")?;
            }
            glue_file.write_all(format!("0x{:x}", c).as_bytes())?;
            if j < mem.data.len() - 1 {
                glue_file.write_all(b", ")?;
            }
        }
        glue_file.write_all(b"\n};\n")?;
    }

    // Write init function
    glue_file.write_all("void init() {\n".as_bytes())?;
    for (i, mem) in dynamic_memories.iter().enumerate() {
        glue_file.write_all(
            format!(
                "memcpy(memory{} + {}, data{}, {});\n",
                mem.index,
                mem.offset,
                i,
                mem.data.len()
            )
            .as_bytes(),
        )?;
    }
    for (_, table) in dynamic_tables.iter().enumerate() {
        glue_file.write_all(
            format!(
                "table{}[{} + {}] = ((uintptr_t) ({}));\n",
                table.index,
                table.offset,
                table.shift,
                get_external_name("functionDef", table.func_index as u32)
            )
            .as_bytes(),
        )?;
    }
    for (i, _) in tables.iter().enumerate() {
        glue_file.write_all(
            format!(
                "  for (int i = 0; i < table{}_length; i++) {{
    table{}[i] = table{}[i] - ((uintptr_t) &tableReferenceBias) - 0x20;
  }}\n",
                i, i, i
            )
            .as_bytes(),
        )?;
    }

    if let Some(function_index) = wasm_module.start {
        glue_file.write_all(format!("  {}(NULL);\n", function_names[function_index as usize]).as_bytes())?;
    }
    glue_file.write_all("}\n".as_bytes())?;

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
    let mut new_name = String::new();
    for e in name.chars() {
        if e == '-' {
            new_name += "_";
        } else if !e.is_ascii_alphanumeric() {
            new_name += &hex::encode(&e.to_string());
        } else {
            new_name += &e.to_string();
        }
    }
    new_name
}

fn convert_func_type_to_c_function(func_type: &wasmparser::FuncType, name: String) -> String {
    if func_type.form != wasmparser::Type::Func || func_type.returns.len() > 1 {
        panic!("Invalid func type: {:?}", func_type);
    }
    let mut fields: Vec<String> = func_type.params.iter().map(|t| wasm_type_to_c_type(*t)).collect();
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
    value: &ConstantOperator,
    global_values: &mut Vec<GlobalValue>,
) -> String {
    let mutable_string = if mutable { "" } else { "const " };
    let type_string = wasm_type_to_c_type(content_type.clone());

    match content_type {
        wasmparser::Type::I32 => {
            if let ConstantOperator::I32Const { value } = value {
                global_values.push(GlobalValue::I32(*value));
                format!(
                    "{}{} {} = {};\n",
                    mutable_string,
                    type_string,
                    get_external_name("global", index as u32),
                    value.to_string()
                )
            } else {
                unimplemented!()
            }
        }
        wasmparser::Type::I64 => {
            if let ConstantOperator::I64Const { value } = value {
                global_values.push(GlobalValue::I64(*value));
                format!(
                    "{}{} {} = {};\n",
                    mutable_string,
                    type_string,
                    get_external_name("global", index as u32),
                    value.to_string()
                )
            } else {
                unimplemented!()
            }
        }
        wasmparser::Type::F32 => {
            if let ConstantOperator::F32Const { value } = value {
                global_values.push(GlobalValue::F32(*value));
                format!(
                    "{}{} {} = {};\n",
                    mutable_string,
                    type_string,
                    get_external_name("global", index as u32),
                    unsafe { std::mem::transmute::<u32, f32>(*value).to_string() }
                )
            } else {
                unimplemented!()
            }
        }
        wasmparser::Type::F64 => {
            if let ConstantOperator::F64Const { value } = value {
                global_values.push(GlobalValue::F64(*value));
                format!(
                    "{}{} {} = {};\n",
                    mutable_string,
                    type_string,
                    get_external_name("global", index as u32),
                    unsafe { std::mem::transmute::<u64, f64>(*value).to_string() }
                )
            } else {
                unimplemented!()
            }
        }
        _ => panic!("Invalid content type: {:?} for global entry", content_type),
    }
}
