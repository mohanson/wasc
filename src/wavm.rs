// pub enum CompileFormat {
//     UnoptimizedLLVMIR,
//     OptimizedLLVMIR,
//     Object,
//     Assembly,
//     PrecompiledWasm,
// }

// impl CompileFormat {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             CompileFormat::UnoptimizedLLVMIR => "unoptimized_llvmir",
//             CompileFormat::OptimizedLLVMIR => "optimized_llvmir",
//             CompileFormat::Object => "object",
//             CompileFormat::Assembly => "assembly",
//             CompileFormat::PrecompiledWasm => "precompiled_wasm",
//         }
//     }
// }

// pub fn compile(
//     source: dyn AsRef<std::path::Path>,
//     format: CompileFormat,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     Ok(())
// }
