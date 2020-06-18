// A C code builder. It will automatically control the indentation by "{" and "}",
// so as to relieve the burden of memory prefix spaces.
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

    // Function write will add indent and "\n" automatically.
    pub fn write(&mut self, line: &str) {
        if line == "}" || line == "};" {
            self.head_whitespace -= 2;
            self.data += &" ".repeat(self.head_whitespace);
            self.data += line;
            self.data += "\n";
            return;
        }
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

    // Building human-friendly arrays.
    pub fn write_array(&mut self, a: Vec<String>, lbreak: u32) {
        let mut l = String::new();
        let mut c: u32 = 0;
        for (i, e) in a.iter().enumerate() {
            l += e;
            c += 1;
            if i != a.len() - 1 {
                l += ", ";
            }
            if c == lbreak {
                self.write(&l);
                l.clear();
                c = 0;
            }
        }
        if !l.is_empty() {
            self.write(&l);
        }
    }
}
