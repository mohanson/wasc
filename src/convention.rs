// use lazy_static::*;

// lazy_static! {
//     pub static ref WASC_BINARY: String = {
//         let current_exe: std::path::PathBuf = std::env::current_exe().unwrap();
//         let current_exe_str: &str = current_exe.as_path().to_str().unwrap();
//         String::from(current_exe_str)
//     };
//     pub static ref WAVM_BINARY: String = {
//         String::from("wavm")
//         // let wasc_binary = std::path::Path::new(WASC_BINARY.as_str());
//         // let wavm_binary = wasc_binary.parent().unwrap().join("wavm");
//         // let wavm_binary_str = wavm_binary.to_str().unwrap();
//         // String::from(wavm_binary_str)
//     };
// }
