use super::code_builder;
use super::context;
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
            _ => panic!("unreachable"),
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

// The initial contents of a table is uninitialized. The elem component of a module defines a vector of element
// segments that initialize a subrange of a table, at a given offset, from a static vector of elements.
#[derive(Debug)]
struct Element {
    table_index: u32,
    offset: Option<ConstantOperator>,
    init: Vec<wasmparser::ElementItem>,
}

// The exports component of a module defines a set of exports that become accessible to the host environment once
// the module has been instantiated.
#[derive(Debug)]
struct Export {
    field: String,
    kind: wasmparser::ExternalKind,
    index: u32,
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
    element_list: Vec<Element>,
    data_list: Vec<Data>,
    start: Option<u32>,
    import_list: Vec<Import>,
    export_list: Vec<Export>,
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
                        wasm_module.custom_list.push(Custom {
                            name: name.to_string(),
                            data: data.to_vec(),
                        });
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
                    wasm_module.global_list.push(Global {
                        global_type: global_type,
                        expr: None,
                    });
                }
                wasmparser::ParserState::EndGlobalSectionEntry => {}
                wasmparser::ParserState::InitExpressionOperator(ref value) => match section_code {
                    Some(wasmparser::SectionCode::Global) => {
                        wasm_module.global_list.last_mut().unwrap().expr = Some(value.clone().into())
                    }
                    Some(wasmparser::SectionCode::Data) => {
                        wasm_module.data_list.last_mut().unwrap().offset = Some(value.clone().into())
                    }
                    Some(wasmparser::SectionCode::Element) => {
                        wasm_module.element_list.last_mut().unwrap().offset = Some(value.clone().into())
                    }
                    _ => {}
                },
                wasmparser::ParserState::BeginActiveDataSectionEntry(memory_index) => {
                    wasm_module.data_list.push(Data {
                        memory_index: memory_index,
                        offset: None,
                        init: vec![],
                    });
                }
                wasmparser::ParserState::EndDataSectionEntry => {}
                wasmparser::ParserState::DataSectionEntryBodyChunk(init) => {
                    wasm_module.data_list.last_mut().unwrap().init = init.to_vec();
                }
                wasmparser::ParserState::BeginElementSectionEntry {
                    table: wasmparser::ElemSectionEntryTable::Active(table_index),
                    ty: wasmparser::Type::AnyFunc,
                } => {
                    wasm_module.element_list.push(Element {
                        table_index: table_index,
                        offset: None,
                        init: vec![],
                    });
                }
                wasmparser::ParserState::EndElementSectionEntry => {}
                wasmparser::ParserState::ElementSectionEntryBody(ref element_list) => {
                    for e in element_list.iter() {
                        match e {
                            wasmparser::ElementItem::Null => {
                                wasm_module
                                    .element_list
                                    .last_mut()
                                    .unwrap()
                                    .init
                                    .push(wasmparser::ElementItem::Null);
                            }
                            wasmparser::ElementItem::Func(func_index) => {
                                wasm_module
                                    .element_list
                                    .last_mut()
                                    .unwrap()
                                    .init
                                    .push(wasmparser::ElementItem::Func(*func_index));
                            }
                        }
                    }
                }
                wasmparser::ParserState::ImportSectionEntry { module, field, ty } => {
                    wasm_module.import_list.push(Import {
                        module: module.to_string(),
                        field: field.to_string(),
                        ty: ty,
                    });
                }
                wasmparser::ParserState::ExportSectionEntry { field, kind, index } => {
                    wasm_module.export_list.push(Export {
                        field: field.to_string(),
                        kind,
                        index,
                    });
                }
                wasmparser::ParserState::Error(ref err) => panic!("{:?}", err),
                _ => {}
            }
        }
        wasm_module
    }
}

// Values are represented by themselves.
#[derive(Debug)]
enum Value {
    I32(i32),
    I64(i64),
    F32(u32),
    F64(u64),
}

// A global instance is the runtime representation of a global variable. It holds an individual value and a flag
// indicating whether it is mutable.
#[derive(Debug)]
struct GlobalInstance {
    global_type: wasmparser::GlobalType,
    value: Option<Value>,
    extern_name: Option<String>,
}

// The store represents all global state that can be manipulated by WebAssembly programs. It consists of the runtime
// representation of all instances of functions, tables, memories, and globals that have been allocated during the
// life time of the abstract machine Syntactically.
//
// Note: only the necessary data information is stored, which is different from the spec.
#[derive(Debug, Default)]
struct Store {
    function_list: Vec<u8>,
    table_list: Vec<u8>,
    memory_list: Vec<u8>,
    global_list: Vec<GlobalInstance>,
}

impl Store {
    fn allocate_global(&mut self, global_instance: GlobalInstance) -> u32 {
        let global_addr = self.global_list.len() as u32;
        self.global_list.push(global_instance);
        global_addr
    }
}

// A module instance is the runtime representation of a module. It is created by instantiating a module, and
// collects runtime representations of all entities that are imported, defined, or exported by the module.
#[derive(Debug, Default)]
struct ModuleInstance {
    type_list: Vec<wasmparser::FuncType>,
    function_addr_list: Vec<u32>,
    table_addr_list: Vec<u32>,
    memory_addr_list: Vec<u32>,
    global_addr_list: Vec<u32>,
    export_list: Vec<u8>,
}

impl ModuleInstance {
    fn from(module: &Module, store: &mut Store) -> Self {
        let mut module_instance = ModuleInstance::default();
        module_instance.type_list = module.type_list.clone();
        // Handle import
        for e in &module.import_list {
            match e.ty {
                wasmparser::ImportSectionEntryType::Function(_) => {}
                wasmparser::ImportSectionEntryType::Memory(_) => {}
                wasmparser::ImportSectionEntryType::Table(_) => {}
                wasmparser::ImportSectionEntryType::Global(global_type) => {
                    let global_addr = store.allocate_global(GlobalInstance {
                        global_type: global_type,
                        value: None,
                        extern_name: Some(format!("{}_{}", e.module, e.field)),
                    });
                    module_instance.global_addr_list.push(global_addr);
                }
            }
        }
        // Let vals be the vector of global initialization values.
        for e in &module.global_list {
            match e.global_type.content_type {
                wasmparser::Type::I32 => {
                    if let Some(ConstantOperator::I32Const { value }) = e.expr {
                        let global_addr = store.allocate_global(GlobalInstance {
                            global_type: e.global_type,
                            value: Some(Value::I32(value)),
                            extern_name: None,
                        });
                        module_instance.global_addr_list.push(global_addr);
                    } else {
                        panic!("unreachable");
                    }
                }
                wasmparser::Type::I64 => {
                    if let Some(ConstantOperator::I64Const { value }) = e.expr {
                        let global_addr = store.allocate_global(GlobalInstance {
                            global_type: e.global_type,
                            value: Some(Value::I64(value)),
                            extern_name: None,
                        });
                        module_instance.global_addr_list.push(global_addr);
                    } else {
                        panic!("unreachable");
                    }
                }
                wasmparser::Type::F32 => {
                    if let Some(ConstantOperator::F32Const { value }) = e.expr {
                        let global_addr = store.allocate_global(GlobalInstance {
                            global_type: e.global_type,
                            value: Some(Value::F32(value)),
                            extern_name: None,
                        });
                        module_instance.global_addr_list.push(global_addr);
                    } else {
                        panic!("unreachable");
                    }
                }
                wasmparser::Type::F64 => {
                    if let Some(ConstantOperator::F64Const { value }) = e.expr {
                        let global_addr = store.allocate_global(GlobalInstance {
                            global_type: e.global_type,
                            value: Some(Value::F64(value)),
                            extern_name: None,
                        });
                        module_instance.global_addr_list.push(global_addr);
                    } else {
                        panic!("unreachable");
                    }
                }
                _ => panic!("unreachable"),
            }
        }
        module_instance
    }
}

// Functions that map between the symbols used for externally visible functions and the function.
fn get_external_name(base_name: &str, index: u32) -> String {
    format!("{}{}", base_name, index)
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
    let mut store = Store::default();
    let wasm_instance = ModuleInstance::from(&wasm_module, &mut store);

    let file_stem = middle.file_stem.clone();
    let object_path = middle.prog_dir.join(file_stem.clone() + ".o");
    let mut object_data: Vec<u8> = vec![];
    for e in wasm_module.custom_list {
        if e.name == "wavm.precompiled_object" {
            object_data.extend_from_slice(&e.data);
        }
    }
    std::fs::write(&object_path, &object_data)?;

    let glue_path = middle.prog_dir.join(file_stem.clone() + "_glue.h");
    let mut glue_file = code_builder::CodeBuilder::place(&glue_path);

    let header_id = format!("{}_GLUE_H", file_stem.to_uppercase());
    glue_file.write(format!(include_str!("glue.template"), header_id, header_id).as_str());

    let mut next_import_index = 0;
    let mut next_import_global_index = 0;
    let mut next_function_index = 0;
    let mut function_entries: Vec<Option<usize>> = vec![];
    let mut function_names: Vec<String> = vec![];
    let mut has_main = false;
    let mut memories: Vec<Vec<u8>> = vec![];
    let mut max_page_num: Option<u32> = None;
    let mut dynamic_memories: Vec<DynamicMemory> = vec![];
    let mut next_global_index = 0;
    let mut tables: Vec<Vec<String>> = vec![];
    let mut table_offset: Option<usize> = None;
    let mut dynamic_table_offset: Option<String> = None;
    let mut dynamic_tables: Vec<DynamicTableEntry> = vec![];

    for i in 0..wasm_instance.type_list.len() {
        glue_file.write(format!("const uint64_t {} = 0;", get_external_name("typeId", i as u32)).as_str());
    }
    for e in wasm_module.global_list {
        glue_file.write(
            generate_global_entry(
                next_global_index,
                &e.global_type.content_type,
                e.global_type.mutable,
                &e.expr.unwrap(),
            )
            .as_str(),
        );
        next_global_index += 1;
    }
    for e in wasm_module.import_list {
        match e.ty {
            wasmparser::ImportSectionEntryType::Function(func_type_index) => {
                function_entries.push(None);
                let func_type = &wasm_instance.type_list[func_type_index as usize];
                let name = format!("wavm_{}_{}", e.module, e.field);
                let import_symbol = get_external_name("functionImport", next_import_index);
                glue_file.write(format!("#define {} {}", name, import_symbol).as_str());
                next_import_index += 1;
                glue_file.write(
                    format!(
                        "extern {};",
                        convert_func_type_to_c_function(&func_type, import_symbol.clone())
                    )
                    .as_str(),
                );
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
                glue_file.write(format!("#define {} {}", name, import_symbol).as_str());
                glue_file.write(format!("extern {} {};", global_type, import_symbol).as_str());
                next_import_global_index += 1;
            }
            _ => {}
        }
    }
    for e in wasm_module.function_list {
        let func_type = &wasm_instance.type_list[e as usize];
        let name = get_external_name("functionDef", next_function_index);
        glue_file.write(format!("extern {};", convert_func_type_to_c_function(&func_type, name.clone())).as_str());
        glue_file.write(
            format!(
                "const uint64_t {} = 0;",
                get_external_name("functionDefMutableDatas", next_function_index)
            )
            .as_str(),
        );
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
                let global_instance =
                    &store.global_list[wasm_instance.global_addr_list[global_index as usize] as usize];
                match global_instance.value {
                    Some(Value::I32(offset)) => {
                        let offset = offset as usize;
                        memories[e.memory_index as usize][offset..offset + e.init.len()].copy_from_slice(&e.init);
                    }
                    None => {
                        let dmemory = DynamicMemory {
                            index: global_index as usize,
                            offset: format!("wavm_{}", global_instance.extern_name.as_ref().unwrap()),
                            data: e.init.to_vec(),
                        };
                        dynamic_memories.push(dmemory);
                    }
                    _ => panic!("unreachable"),
                }
            }
            _ => {}
        }
    }

    for e in wasm_module.export_list {
        match e.kind {
            wasmparser::ExternalKind::Function => {
                let function_index = function_entries[e.index as usize].expect("Exported function should exist!");
                glue_file.write(
                    format!(
                        "#define wavm_exported_function_{} {}",
                        convert_func_name_to_c_function(&e.field),
                        get_external_name("functionDef", function_index as u32),
                    )
                    .as_str(),
                );

                if &e.field == "_start" {
                    has_main = true;
                }
            }
            _ => {}
        }
    }

    for e in wasm_module.element_list {
        match e.offset {
            Some(ConstantOperator::I32Const { value }) => {
                table_offset = Some(value as usize);
            }
            Some(ConstantOperator::GlobalGet { global_index }) => {
                let global_instance =
                    &store.global_list[wasm_instance.global_addr_list[global_index as usize] as usize];
                match global_instance.value {
                    Some(Value::I32(offset)) => table_offset = Some(offset as usize),
                    None => {
                        dynamic_table_offset = Some(format!("wavm_{}", global_instance.extern_name.as_ref().unwrap()))
                    }
                    _ => panic!("unreachable"),
                }
            }
            _ => panic!("unreachable"),
        }

        let index = e.table_index as usize;
        if let Some(x) = dynamic_table_offset.clone() {
            for (i, item) in e.init.iter().enumerate() {
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
            for (i, item) in e.init.iter().enumerate() {
                if let wasmparser::ElementItem::Func(func_index) = item {
                    tables[index][offset + i] =
                        format!("((uintptr_t) ({}))", get_external_name("functionDef", *func_index));
                }
            }
        }
    }

    for (i, table) in tables.iter().enumerate() {
        glue_file.write(format!("uint32_t table{}_length = {};", i, table.len()).as_str());
        glue_file.write(format!("uintptr_t table{}[{}] = {{", i, table.len()).as_str());
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
            if j < striped_table.len() - 1 {
                glue_file.write(format!("{},", c).as_str());
            } else {
                glue_file.write(c);
            }
        }
        glue_file.write("};");
        glue_file.write(
            format!(
                "uintptr_t* {} = table{};",
                get_external_name("tableOffset", i as u32),
                i
            )
            .as_str(),
        );
        glue_file.write(format!("#define TABLE{}_DEFINED 1", i).as_str());
    }

    for (i, mem) in memories.iter().enumerate() {
        glue_file.write(format!("uint32_t memory{}_length = {};", i, mem.len()).as_str());
        glue_file.write(
            format!(
                "uint8_t __attribute__((section (\".wasm_memory\"))) memory{}[{}] = {{",
                i,
                mem.len()
            )
            .as_str(),
        );
        let reversed_striped_mem: Vec<u8> = mem.iter().rev().map(|x| *x).skip_while(|c| *c == 0).collect();
        let mut striped_mem: Vec<u8> = reversed_striped_mem.into_iter().rev().collect();
        if striped_mem.len() == 0 {
            striped_mem.push(0);
        }
        for (j, c) in striped_mem.iter().enumerate() {
            if j < striped_mem.len() - 1 {
                glue_file.write(format!("0x{:x},", c).as_str());
            } else {
                glue_file.write(format!("0x{:x}", c).as_str());
            }
        }
        glue_file.write("};");
        glue_file.write(
            format!(
                "struct memory_runtime_data {} = {{memory{}, {}}};",
                get_external_name("memoryOffset", i as u32),
                i,
                mem.len() / 65536
            )
            .as_str(),
        );
        glue_file.write(format!("#define MEMORY{}_DEFINED 1", i).as_str());
        if let Some(x) = max_page_num {
            glue_file.write(format!("#define WAVM_MAX_PAGE {}", x.to_string()).as_str());
        }
    }

    for (i, mem) in dynamic_memories.iter().enumerate() {
        glue_file.write(format!("uint32_t data{}_length = {};", i, mem.data.len()).as_str());
        glue_file.write(format!("uint8_t data{}[{}] = {{", i, mem.data.len()).as_str());
        for (j, c) in mem.data.iter().enumerate() {
            if j < mem.data.len() - 1 {
                glue_file.write(format!("0x{:x},", c).as_str());
            } else {
                glue_file.write(format!("0x{:x}", c).as_str());
            }
        }
        glue_file.write("};");
    }

    // Write init function
    glue_file.write("void init() {");
    for (i, mem) in dynamic_memories.iter().enumerate() {
        glue_file.write(
            format!(
                "memcpy(memory{} + {}, data{}, {});",
                mem.index,
                mem.offset,
                i,
                mem.data.len()
            )
            .as_str(),
        );
    }
    for (_, table) in dynamic_tables.iter().enumerate() {
        glue_file.write(
            format!(
                "table{}[{} + {}] = ((uintptr_t) ({}));",
                table.index,
                table.offset,
                table.shift,
                get_external_name("functionDef", table.func_index as u32)
            )
            .as_str(),
        );
    }
    for (i, _) in tables.iter().enumerate() {
        glue_file.write(format!("for (int i = 0; i < table{}_length; i++) {{", i).as_str());
        glue_file.write(
            format!(
                "table{}[i] = table{}[i] - ((uintptr_t) &tableReferenceBias) - 0x20;",
                i, i
            )
            .as_str(),
        );
        glue_file.write("}");
    }

    if let Some(function_index) = wasm_module.start {
        glue_file.write(format!("{}(NULL);", function_names[function_index as usize]).as_str());
    }
    glue_file.write("}");

    if has_main {
        glue_file.write("int main() {");
        glue_file.write("wavm_exported_function__start(NULL);");
        glue_file.write("return -1;");
        glue_file.write("}");
    }

    glue_file.write(format!("#endif /* {} */", header_id).as_str());
    glue_file.close()?;

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
) -> String {
    let mutable_string = if mutable { "" } else { "const " };
    let type_string = wasm_type_to_c_type(content_type.clone());

    match content_type {
        wasmparser::Type::I32 => {
            if let ConstantOperator::I32Const { value } = value {
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
