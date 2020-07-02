#[derive(Clone, Debug)]
pub enum Platform {
    PosixX8664,
    PosixX8664Spectest,
    PosixX8664Wasi,
    Unknown,
}

// A Config specifies the global config for a build.
#[derive(Clone, Debug)]
pub struct Config {
    // Path of cc, usually the result of "$ which gcc".
    pub binary_cc: String,
    pub binary_wavm: String,
    // Platfrom flag and their files.
    pub platform: Platform,
    pub platform_posix_x86_64_h: &'static str,
    pub platform_posix_x86_64_runtime_s: &'static str,
    pub platform_posix_x86_64_spectest_h: &'static str,
    pub platform_posix_x86_64_spectest_runtime_s: &'static str,
    pub platform_posix_x86_64_wasi_h: &'static str,
    pub platform_posix_x86_64_wasi_runtime_s: &'static str,
    pub platform_common_wavm_h: &'static str,
    pub platform_common_wasi_h: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            binary_cc: String::from("gcc"),
            binary_wavm: String::from("wavm"),
            platform: Platform::Unknown,
            platform_posix_x86_64_h: include_str!("./platform/posix_x86_64.h"),
            platform_posix_x86_64_runtime_s: include_str!("./platform/posix_x86_64_runtime.S"),
            platform_posix_x86_64_spectest_h: include_str!("./platform/posix_x86_64_spectest.h"),
            platform_posix_x86_64_spectest_runtime_s: include_str!("./platform/posix_x86_64_spectest_runtime.S"),
            platform_posix_x86_64_wasi_h: include_str!("./platform/posix_x86_64_wasi.h"),
            platform_posix_x86_64_wasi_runtime_s: include_str!("./platform/posix_x86_64_wasi_runtime.S"),
            platform_common_wavm_h: include_str!("./platform/common/wavm.h"),
            platform_common_wasi_h: include_str!("./platform/common/wasi.h"),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Middle {
    // Config is the global config for a build.
    pub config: Config,

    // CurrentDir is the caller's working directory, or the empty string to use
    // the current directory of the running process.
    pub current_dir: std::path::PathBuf,

    // Source wasm/wast file.
    pub file: std::path::PathBuf,

    // File stem is the source wasm/wast file's name without extension.
    // Example: file_stem(helloworld.wasm) => helloworld
    pub file_stem: String,

    // Template path.
    pub path_prog: std::path::PathBuf,                        // xx_build
    pub path_platform_code_folder: std::path::PathBuf,        // xx_build/platform
    pub path_platform_common_code_folder: std::path::PathBuf, // xx_build/platform/common
    pub path_platform_common_wavm_h: std::path::PathBuf,      // xx_build/platform/common/wavm.h
    pub path_platform_common_wasi_h: std::path::PathBuf,      // xx_build/platform/common/wasi.h
    pub path_platform_header: std::path::PathBuf,             // xx_build/platform/xx.h
    pub path_platform_s: std::path::PathBuf,                  // xx_build/platform/xx_runtime.s
    pub path_object: std::path::PathBuf,                      // xx_build/xx.o
    pub path_glue: std::path::PathBuf,                        // xx_build/xx_glue.h
    pub path_c: std::path::PathBuf,                           // xx_build/xx.c
    pub path_precompiled: std::path::PathBuf,                 // xx_build/xx_precompiled.wasm
}

impl Middle {
    // Set global config for middle.
    pub fn init_config(&mut self, config: Config) {
        self.config = config;
    }

    // Initialize the compilation environment.
    pub fn init_file<P: AsRef<std::path::Path>>(&mut self, p: P) {
        self.current_dir = std::env::current_dir().unwrap();
        self.file = p.as_ref().to_path_buf();
        self.file_stem = self.file.file_stem().unwrap().to_str().unwrap().to_string();
        self.path_prog = self.file.with_file_name(format!("{}_build", self.file_stem));
        self.path_platform_code_folder = self.path_prog.join("platform");
        self.path_platform_common_code_folder = self.path_platform_code_folder.join("common");
        match self.config.platform {
            Platform::PosixX8664 => {
                self.path_platform_header = self.path_platform_code_folder.join("posix_x86_64.h");
                self.path_platform_s = self.path_platform_code_folder.join("posix_x86_64_runtime.S");
            }
            Platform::PosixX8664Spectest => {
                self.path_platform_header = self.path_platform_code_folder.join("posix_x86_64_spectest.h");
                self.path_platform_s = self.path_platform_code_folder.join("posix_x86_64_spectest_runtime.S");
            }
            Platform::PosixX8664Wasi => {
                self.path_platform_header = self.path_platform_code_folder.join("posix_x86_64_wasi.h");
                self.path_platform_s = self.path_platform_code_folder.join("posix_x86_64_wasi_runtime.S");
            }
            Platform::Unknown => {
                panic!("unreachable");
            }
        }
        self.path_object = self.path_prog.join(self.file_stem.clone() + ".o");
        self.path_glue = self.path_prog.join(self.file_stem.clone() + "_glue.h");
        self.path_c = self.path_prog.join(self.file_stem.clone() + ".c");
        self.path_precompiled = self.path_prog.join(self.file_stem.clone() + "_precompiled.wasm");
        self.path_platform_common_wavm_h = self.path_platform_common_code_folder.join("wavm.h");
        self.path_platform_common_wasi_h = self.path_platform_common_code_folder.join("wasi.h");
    }
}
