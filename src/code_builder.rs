// A C code builder.
pub struct CodeBuilder {
    path: std::path::PathBuf,
    data: String,
    head_whitespace: usize,
}

impl CodeBuilder {
    pub fn place<P: AsRef<std::path::Path>>(path: P) -> Self {
        CodeBuilder {
            path: path.as_ref().to_path_buf(),
            data: String::new(),
            head_whitespace: 0,
        }
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(&self.path, &self.data)?;
        Ok(())
    }

    pub fn write(&mut self, line: &str) {
        self.data += &" ".repeat(self.head_whitespace);
        self.data += line;
        self.data += "\n";
        if line.ends_with("{") {
            self.head_whitespace += 2;
        }
        if line.ends_with("}") {
            self.head_whitespace -= 2;
        }
    }
}
