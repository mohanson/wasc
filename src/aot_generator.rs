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
#[derive(Clone, Debug)]
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
enum GlobalInstance {
    Wasm {
        global_type: wasmparser::GlobalType,
        value: Value,
    },
    Host {
        global_type: wasmparser::GlobalType,
        import_name: String,
    },
}

// A function instance is the runtime representation of a function. It effectively is a closure of the original
// function over the runtime module instance of its originating module. The module instance is used to resolve
// references to other definitions during execution of the function.
#[derive(Debug)]
enum FunctionInstance {
    WasmFunc {
        function_type: wasmparser::FuncType,
    },
    // A host function is a function expressed outside WebAssembly but passed to a module as an import.
    HostFunc {
        function_type: wasmparser::FuncType,
        import_name: String,
    },
}

// A memory instance is the runtime representation of a linear memory. It holds a vector of bytes and an optional
// maximum size, if one was specified at the definition site of the memory.
#[derive(Debug)]
enum MemoryInstance {
    Wasm {
        memory_type: wasmparser::MemoryType,
        data: Vec<Data>,
    },
    Host {
        memory_type: wasmparser::MemoryType,
        data: Vec<Data>,
        import_name: String,
    },
}

// A table instance is the runtime representation of a table. It holds a vector of function elements and an optional
// maximum size, if one was specified in the table type at the table’s definition site.
#[derive(Debug)]
enum TableInstance {
    Wasm {
        table_type: wasmparser::TableType,
        element_list: Vec<Element>,
    },
    Host {
        table_type: wasmparser::TableType,
        element_list: Vec<Element>,
        import_name: String,
    },
}

// The store represents all global state that can be manipulated by WebAssembly programs. It consists of the runtime
// representation of all instances of functions, tables, memories, and globals that have been allocated during the
// life time of the abstract machine Syntactically.
//
// Note: only the necessary data information is stored, which is different from the spec.
#[derive(Debug, Default)]
struct Store {
    function_list: Vec<FunctionInstance>,
    table_list: Vec<TableInstance>,
    memory_list: Vec<MemoryInstance>,
    global_list: Vec<GlobalInstance>,
}

impl Store {
    fn allocate_memory(&mut self, memory_instance: MemoryInstance) -> u32 {
        let memory_addr = self.memory_list.len() as u32;
        self.memory_list.push(memory_instance);
        memory_addr
    }

    fn allocate_function(&mut self, function_instance: FunctionInstance) -> u32 {
        let function_addr = self.function_list.len() as u32;
        self.function_list.push(function_instance);
        function_addr
    }

    fn allocate_table(&mut self, table_instance: TableInstance) -> u32 {
        let table_addr = self.table_list.len() as u32;
        self.table_list.push(table_instance);
        table_addr
    }

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
    export_list: Vec<Export>,
}

impl ModuleInstance {
    fn from(module: &Module, store: &mut Store) -> Self {
        let mut module_instance = ModuleInstance::default();
        module_instance.type_list = module.type_list.clone();
        module_instance.export_list = module.export_list.clone();
        // Handle import
        for e in &module.import_list {
            let import_name = format!("{}_{}", e.module, e.field);
            match e.ty {
                wasmparser::ImportSectionEntryType::Function(function_type_index) => {
                    let function_type = &module_instance.type_list[function_type_index as usize];
                    let function_addr = store.allocate_function(FunctionInstance::HostFunc {
                        function_type: function_type.clone(),
                        import_name: import_name,
                    });
                    module_instance.function_addr_list.push(function_addr);
                }
                wasmparser::ImportSectionEntryType::Memory(memory_type) => {
                    let memory_addr = store.allocate_memory(MemoryInstance::Host {
                        memory_type: memory_type,
                        data: vec![],
                        import_name: import_name,
                    });
                    module_instance.memory_addr_list.push(memory_addr);
                }
                wasmparser::ImportSectionEntryType::Table(table_type) => {
                    let table_addr = store.allocate_table(TableInstance::Host {
                        table_type: table_type,
                        element_list: vec![],
                        import_name: import_name,
                    });
                    module_instance.table_addr_list.push(table_addr);
                }
                wasmparser::ImportSectionEntryType::Global(global_type) => {
                    let global_addr = store.allocate_global(GlobalInstance::Host {
                        global_type: global_type,
                        import_name: import_name,
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
                        let global_addr = store.allocate_global(GlobalInstance::Wasm {
                            global_type: e.global_type,
                            value: Value::I32(value),
                        });
                        module_instance.global_addr_list.push(global_addr);
                    } else {
                        panic!("unreachable");
                    }
                }
                wasmparser::Type::I64 => {
                    if let Some(ConstantOperator::I64Const { value }) = e.expr {
                        let global_addr = store.allocate_global(GlobalInstance::Wasm {
                            global_type: e.global_type,
                            value: Value::I64(value),
                        });
                        module_instance.global_addr_list.push(global_addr);
                    } else {
                        panic!("unreachable");
                    }
                }
                wasmparser::Type::F32 => {
                    if let Some(ConstantOperator::F32Const { value }) = e.expr {
                        let global_addr = store.allocate_global(GlobalInstance::Wasm {
                            global_type: e.global_type,
                            value: Value::F32(value),
                        });
                        module_instance.global_addr_list.push(global_addr);
                    } else {
                        panic!("unreachable");
                    }
                }
                wasmparser::Type::F64 => {
                    if let Some(ConstantOperator::F64Const { value }) = e.expr {
                        let global_addr = store.allocate_global(GlobalInstance::Wasm {
                            global_type: e.global_type,
                            value: Value::F64(value),
                        });
                        module_instance.global_addr_list.push(global_addr);
                    } else {
                        panic!("unreachable");
                    }
                }
                _ => panic!("unreachable"),
            }
        }
        // Allocate each function in module.function_list
        for e in &module.function_list {
            let function_type = &module_instance.type_list[*e as usize];
            let function_addr = store.allocate_function(FunctionInstance::WasmFunc {
                function_type: function_type.clone(),
            });
            module_instance.function_addr_list.push(function_addr);
        }
        // Allocate each table in module.table_list
        for e in &module.table_list {
            let table_addr = store.allocate_table(TableInstance::Wasm {
                table_type: *e,
                element_list: vec![],
            });
            module_instance.table_addr_list.push(table_addr);
        }
        // Allocate each memory in module.memory_list
        for e in &module.memory_list {
            let memory_addr = store.allocate_memory(MemoryInstance::Wasm {
                memory_type: *e,
                data: vec![],
            });
            module_instance.memory_addr_list.push(memory_addr);
        }
        module_instance
    }
}

// Functions that map between the symbols used for externally visible functions and the function.
fn get_external_name(base_name: &str, index: u32) -> String {
    format!("{}{}", base_name, index)
}

// Since the naming rules supported by C and wasm are different, the wasm naming needs to be converted.
pub fn cnaming(name: &str) -> String {
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

// Emit wasm type to c code.
fn emit_type(t: wasmparser::Type) -> String {
    match t {
        wasmparser::Type::I32 => "int32_t".to_string(),
        wasmparser::Type::I64 => "int64_t".to_string(),
        wasmparser::Type::F32 => "float".to_string(),
        wasmparser::Type::F64 => "double".to_string(),
        _ => panic!("unreachable"),
    }
}

// Emit wasm function type to c function signature.
fn emit_function_signature(func_type: &wasmparser::FuncType, name: String) -> String {
    if func_type.form != wasmparser::Type::Func || func_type.returns.len() > 1 {
        panic!("unreachable");
    }
    let mut fields: Vec<String> = func_type.params.iter().map(|t| emit_type(*t)).collect();
    fields.insert(0, "void*".to_string());
    let return_type = if func_type.returns.len() > 0 {
        format!("wavm_ret_{}", emit_type(func_type.returns[0]))
    } else {
        "void*".to_string()
    };
    format!("{} ({}) ({})", return_type, name, fields.join(", ")).to_string()
}

// Emit memory data with static/dynamic offset.
fn emit_memory_data(mi: u32, di: u32, offset: &str, len: u32) -> String {
    format!("memcpy(memory{} + {}, memory{}_data{}, {});", mi, offset, mi, di, len)
}

pub fn generate(middle: &mut context::Middle) -> Result<(), Box<dyn std::error::Error>> {
    let wasm_data: Vec<u8> = std::fs::read(middle.wavm_precompiled_wasm.to_str().unwrap())?;
    let wasm_module = Module::from(wasm_data.clone());
    let mut store = Store::default();
    let wasm_instance = ModuleInstance::from(&wasm_module, &mut store);

    let file_stem = middle.file_stem.clone();
    // Save precompiled object.
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
    glue_file.write(format!(include_str!("glue.template"), header_id, header_id));

    // Emit type.
    for i in 0..wasm_instance.type_list.len() {
        glue_file.write(format!("const uint64_t {} = 0;", get_external_name("typeId", i as u32)));
    }
    // Emit global.
    for i in &wasm_instance.global_addr_list {
        let global_instance = &store.global_list[*i as usize];
        let extern_name = get_external_name("global", *i);

        match global_instance {
            GlobalInstance::Wasm { global_type, value } => {
                let type_string = emit_type(global_type.content_type.clone());
                let mutable_string = if global_type.mutable { "" } else { "const " };
                match value {
                    Value::I32(v) => {
                        glue_file.write(format!("{}{} {} = {};", mutable_string, type_string, extern_name, v));
                    }
                    Value::I64(v) => {
                        glue_file.write(format!("{}{} {} = {};", mutable_string, type_string, extern_name, v));
                    }
                    Value::F32(v) => {
                        let f = unsafe { std::mem::transmute::<u32, f32>(*v).to_string() };
                        glue_file.write(format!("{}{} {} = {};", mutable_string, type_string, extern_name, f));
                    }
                    Value::F64(v) => {
                        let f = unsafe { std::mem::transmute::<u64, f64>(*v).to_string() };
                        glue_file.write(format!("{}{} {} = {};", mutable_string, type_string, extern_name, f));
                    }
                }
            }
            GlobalInstance::Host {
                global_type,
                import_name,
            } => {
                let type_string = emit_type(global_type.content_type.clone());
                let wavm_name = format!("wavm_{}", import_name);
                glue_file.write(format!("#define {} {}", wavm_name, extern_name));
                glue_file.write(format!("extern {} {};", type_string, extern_name));
            }
        }
    }
    // Emit function.
    let mut wasm_function_counter = 0;
    let mut host_function_counter = 0;
    let mut function_name_list: Vec<String> = vec![];
    for i in &wasm_instance.function_addr_list {
        let function_instance = &store.function_list[*i as usize];
        match function_instance {
            FunctionInstance::WasmFunc { function_type } => {
                let name = get_external_name("functionDef", wasm_function_counter);
                glue_file.write(format!(
                    "extern {};",
                    emit_function_signature(&function_type, name.clone())
                ));
                let a = get_external_name("functionDefMutableDatas", wasm_function_counter);
                glue_file.write(format!("const uint64_t {} = 0;", a));
                wasm_function_counter += 1;
                function_name_list.push(name);
            }
            FunctionInstance::HostFunc {
                function_type,
                import_name,
            } => {
                let extern_name = get_external_name("functionImport", host_function_counter);
                let signature = emit_function_signature(&function_type, extern_name.clone());
                glue_file.write(format!("#define wavm_{} {}", import_name, extern_name));
                glue_file.write(format!("extern {};", signature));
                function_name_list.push(format!("wavm_{}", import_name));
                host_function_counter += 1
            }
        }
    }
    // Get the correspondence between data and memory.
    for e in wasm_module.data_list {
        let memory_instance = &mut store.memory_list[wasm_instance.memory_addr_list[e.memory_index as usize] as usize];
        match memory_instance {
            MemoryInstance::Wasm { memory_type: _, data } => {
                data.push(e);
            }
            MemoryInstance::Host {
                memory_type: _,
                data,
                import_name: _,
            } => {
                data.push(e);
            }
        }
    }
    // Emit memory.
    let mut init_function_list: Vec<String> = vec![];
    for i in wasm_instance.memory_addr_list {
        let memory_instance = &store.memory_list[i as usize];
        match memory_instance {
            MemoryInstance::Wasm { memory_type, data } => {
                glue_file.write(format!("uint8_t* memory{};", i));
                let extern_name = get_external_name("memoryOffset", i as u32);
                glue_file.write(format!("struct memory_instance {};", extern_name));
                if let Some(x) = memory_type.limits.maximum {
                    glue_file.write(format!("#define MEMORY{}_MAX_PAGE {}", i, x));
                }
                for (j, e) in data.iter().enumerate() {
                    glue_file.write(format!("uint8_t memory{}_data{}[{}] = {{", i, j, e.init.len()));
                    let array: Vec<String> = e.init.iter().map(|x| format!("0x{:02x}", x)).collect();
                    glue_file.write_array(array, 16);
                    glue_file.write("};");
                }
                glue_file.write(format!("#define MEMORY{}_DEFINED 1", i));
                glue_file.write(format!("void init_memory{}() {{", i));
                let memory_size = memory_type.limits.initial * 65536;
                glue_file.write(format!("memory{} = calloc({}, 1);", i, memory_size));
                for (j, e) in data.iter().enumerate() {
                    match e.offset {
                        Some(ConstantOperator::I32Const { value }) => {
                            let a = emit_memory_data(i, j as u32, value.to_string().as_str(), e.init.len() as u32);
                            glue_file.write(a);
                        }
                        Some(ConstantOperator::GlobalGet { global_index }) => {
                            let global_addr = wasm_instance.global_addr_list[global_index as usize];
                            let global_instance = &store.global_list[global_addr as usize];
                            match global_instance {
                                GlobalInstance::Wasm { global_type: _, value } => match value {
                                    Value::I32(value) => {
                                        let offset = value.to_string();
                                        let len = e.init.len() as u32;
                                        let a = emit_memory_data(i, j as u32, offset.as_str(), len);
                                        glue_file.write(a);
                                    }
                                    _ => panic!("unreachable"),
                                },
                                GlobalInstance::Host {
                                    global_type: _,
                                    import_name,
                                } => {
                                    let offset = format!("wavm_{}", import_name);
                                    let len = e.init.len() as u32;
                                    let a = emit_memory_data(i, j as u32, offset.as_str(), len);
                                    glue_file.write(a);
                                }
                            }
                        }
                        _ => panic!("unreachable"),
                    }
                }
                glue_file.write(format!("{}.base = memory{};", extern_name, i));
                glue_file.write(format!("{}.num_pages = {};", extern_name, memory_type.limits.initial));
                glue_file.write("}");
                init_function_list.push(format!("init_memory{}", i));
            }
            MemoryInstance::Host {
                memory_type: _,
                data: _,
                import_name: _,
            } => {
                // Does it make sense to support it?
            }
        }
    }
    // Get the correspondence between elem and table.
    for e in wasm_module.element_list {
        let table_instance = &mut store.table_list[wasm_instance.table_addr_list[e.table_index as usize] as usize];
        match table_instance {
            TableInstance::Wasm {
                table_type: _,
                element_list,
            } => {
                element_list.push(e);
            }
            TableInstance::Host {
                table_type: _,
                element_list,
                import_name: _,
            } => {
                element_list.push(e);
            }
        }
    }
    // Emit table.
    for i in wasm_instance.table_addr_list {
        let table_instance = &store.table_list[i as usize];
        match table_instance {
            TableInstance::Wasm {
                table_type,
                element_list,
            } => {
                glue_file.write(format!("uint32_t table{}_length = {};", i, table_type.limits.initial));
                let mut table: Vec<String> = vec!["0".into(); table_type.limits.initial as usize];
                let mut space: Vec<String> = vec![];
                for e in element_list {
                    match e.offset {
                        Some(ConstantOperator::I32Const { value }) => {
                            for (j, item) in e.init.iter().enumerate() {
                                match item {
                                    wasmparser::ElementItem::Func(func_index) => {
                                        let extern_name = get_external_name("functionDef", *func_index);
                                        let table_item = format!("((uintptr_t) ({}))", extern_name);
                                        table[value as usize + j] = table_item;
                                    }
                                    wasmparser::ElementItem::Null => panic!("unreachable"),
                                }
                            }
                        }
                        Some(ConstantOperator::GlobalGet { global_index }) => {
                            let global_addr = wasm_instance.global_addr_list[global_index as usize];
                            let global_instance = &store.global_list[global_addr as usize];
                            match global_instance {
                                GlobalInstance::Wasm { global_type: _, value } => match value {
                                    Value::I32(value) => {
                                        for (j, item) in e.init.iter().enumerate() {
                                            match item {
                                                wasmparser::ElementItem::Func(func_index) => {
                                                    let extern_name = get_external_name("functionDef", *func_index);
                                                    let table_item = format!("((uintptr_t) ({}))", extern_name);
                                                    table[*value as usize + j] = table_item;
                                                }
                                                wasmparser::ElementItem::Null => panic!("unreachable"),
                                            }
                                        }
                                    }
                                    _ => panic!("unreachable"),
                                },
                                GlobalInstance::Host {
                                    global_type: _,
                                    import_name,
                                } => {
                                    for (j, item) in e.init.iter().enumerate() {
                                        match item {
                                            wasmparser::ElementItem::Func(func_index) => {
                                                space.push(format!(
                                                    "table{}[{} + {}] = ((uintptr_t) ({}));",
                                                    i,
                                                    format!("wavm_{}", import_name),
                                                    j,
                                                    get_external_name("functionDef", *func_index as u32)
                                                ));
                                            }
                                            wasmparser::ElementItem::Null => panic!("unreachable"),
                                        }
                                    }
                                }
                            }
                        }
                        _ => panic!("unreachable"),
                    }
                }

                glue_file.write(format!("uintptr_t table{}[{}] = {{", i, table_type.limits.initial));
                glue_file.write_array(table, 4);
                glue_file.write("};");
                glue_file.write(format!("uintptr_t* tableOffset{} = table{};", i, i));
                glue_file.write(format!("#define TABLE{}_DEFINED 1", i));
                glue_file.write(format!("void init_table{}() {{", i));
                for e in space {
                    glue_file.write(&e);
                }
                glue_file.write(format!("for (int i = 0; i < table{}_length; i++) {{", i));
                glue_file.write(format!(
                    "table{}[i] = table{}[i] - ((uintptr_t) &tableReferenceBias) - 0x20;",
                    i, i
                ));
                glue_file.write("}");
                glue_file.write("}");
                init_function_list.push(format!("init_table{}", i));
            }
            TableInstance::Host {
                table_type: _,
                element_list,
                import_name,
            } => {
                let extern_name = get_external_name("table", i);
                glue_file.write(format!("#define wavm_{}_length {}_length", import_name, extern_name));
                glue_file.write(format!("extern uint32_t table{}_length;", i));
                glue_file.write(format!("#define wavm_{} {}", import_name, extern_name));
                glue_file.write(format!("extern uintptr_t table{}[];", i));
                glue_file.write(format!("uintptr_t* tableOffset{} = table{};", i, i));
                glue_file.write(format!("#define TABLE{}_DEFINED 1", i));
                glue_file.write(format!("void init_table{}() {{", i));
                for e in element_list {
                    for (j, item) in e.init.iter().enumerate() {
                        let offset: String = match e.offset {
                            Some(ConstantOperator::I32Const { value }) => value.to_string(),
                            Some(ConstantOperator::GlobalGet { global_index }) => {
                                let global_addr = wasm_instance.global_addr_list[global_index as usize];
                                let global_instance = &store.global_list[global_addr as usize];
                                match global_instance {
                                    GlobalInstance::Wasm { global_type: _, value } => match value {
                                        Value::I32(value) => value.to_string(),
                                        _ => panic!("unreachable"),
                                    },
                                    GlobalInstance::Host {
                                        global_type: _,
                                        import_name,
                                    } => format!("wavm_{}", import_name),
                                }
                            }
                            _ => panic!("unreachable"),
                        };
                        match item {
                            wasmparser::ElementItem::Func(func_index) => {
                                glue_file.write(format!(
                                    "table{}[{} + {}] = ((uintptr_t) ({}));",
                                    i,
                                    offset,
                                    j,
                                    get_external_name("functionDef", *func_index)
                                ));
                            }
                            wasmparser::ElementItem::Null => panic!("unreachable"),
                        }
                    }
                }
                glue_file.write(format!("for (int i = 0; i < table{}_length; i++) {{", i));
                glue_file.write(format!(
                    "table{}[i] = table{}[i] - ((uintptr_t) &tableReferenceBias) - 0x20;",
                    i, i
                ));
                glue_file.write("}");
                glue_file.write("}");
                init_function_list.push(format!("init_table{}", i));
            }
        }
    }
    // Emit export.
    let mut has_main = false;
    for e in wasm_instance.export_list {
        match e.kind {
            wasmparser::ExternalKind::Function => {
                glue_file.write(format!(
                    "#define wavm_exported_function_{} {}",
                    cnaming(&e.field),
                    function_name_list[e.index as usize],
                ));
                if &e.field == "_start" {
                    has_main = true;
                }
            }
            _ => {}
        }
    }
    // Emit init function.
    glue_file.write("void init() {");
    for e in init_function_list {
        glue_file.write(format!("{}();", e));
    }
    if let Some(function_index) = wasm_module.start {
        glue_file.write(format!("{}(NULL);", function_name_list[function_index as usize]));
    }
    glue_file.write("}");
    // Emit main function.
    if has_main {
        glue_file.write("int main() {");
        glue_file.write("init();");
        glue_file.write("wavm_exported_function__start(NULL);");
        glue_file.write("return -1;");
        glue_file.write("}");
    }
    glue_file.write(format!("#endif /* {} */", header_id));
    glue_file.close()?;

    middle.aot_object = object_path;
    middle.aot_glue = glue_path;
    Ok(())
}
