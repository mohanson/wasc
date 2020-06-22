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
    pub platform_posix_x86_64: &'static str,
    pub platform_posix_x86_64_runtime: &'static str,
    pub platform_posix_x86_64_spectest: &'static str,
    pub platform_posix_x86_64_spectest_runtime: &'static str,
    pub platform_posix_x86_64_wasi: &'static str,
    pub platform_posix_x86_64_wasi_runtime: &'static str,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            binary_cc: String::from("gcc"),
            binary_wavm: String::from("wavm"),
            platform: Platform::Unknown,
            platform_posix_x86_64: include_str!("./platform/posix_x86_64.h"),
            platform_posix_x86_64_runtime: include_str!("./platform/posix_x86_64_runtime.S"),
            platform_posix_x86_64_spectest: include_str!("./platform/posix_x86_64_spectest.h"),
            platform_posix_x86_64_spectest_runtime: include_str!("./platform/posix_x86_64_spectest_runtime.S"),
            platform_posix_x86_64_wasi: include_str!("./platform/posix_x86_64_wasi.h"),
            platform_posix_x86_64_wasi_runtime: include_str!("./platform/posix_x86_64_wasi_runtime.S"),
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
    pub path_prog: std::path::PathBuf,
    pub path_platform_code_folder: std::path::PathBuf,
    pub path_platform_header: std::path::PathBuf,
    pub path_platform_s: std::path::PathBuf,
    pub path_object: std::path::PathBuf,
    pub path_glue: std::path::PathBuf,
    pub path_c: std::path::PathBuf,
    pub path_precompiled: std::path::PathBuf,
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
        self.path_prog = self.file.parent().unwrap().to_path_buf();
        if self.path_prog.parent() == None {
            self.path_prog = std::path::PathBuf::from("./");
        }
        self.path_platform_code_folder = self.path_prog.join(self.file_stem.clone() + "_platform");
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
                // Must specify the target platform in advance, from environment variables, or command line parameters,
                // or guess.
                panic!("unreachable");
            }
        }
        self.path_object = self.path_prog.join(self.file_stem.clone() + ".o");
        self.path_glue = self.path_prog.join(self.file_stem.clone() + "_glue.h");
        self.path_c = self.path_prog.join(self.file_stem.clone() + ".c");
        self.path_precompiled = self.path_prog.join(self.file_stem.clone() + "_precompiled.wasm");
    }
}
