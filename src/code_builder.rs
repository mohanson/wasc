// A C code builder. It will automatically control the indentation by "{" and "}",
// so as to relieve the burden of memory prefix spaces.
pub struct CodeBuilder {
    pub path: std::path::PathBuf,
    pub data: String,
    head_whitespace: usize,
}

impl CodeBuilder {
    pub fn create<P: AsRef<std::path::Path>>(path: P) -> Self {
        CodeBuilder {
            path: path.as_ref().to_path_buf(),
            data: String::new(),
            head_whitespace: 0,
        }
    }

    pub fn append<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut c = CodeBuilder::create(&path);
        c.data = std::str::from_utf8(&std::fs::read(&path)?)?.to_string();
        return Ok(c);
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::write(&self.path, &self.data)?;
        Ok(())
    }

    // Function write will add indent and "\n" automatically.
    pub fn write<P: AsRef<str>>(&mut self, line: P) {
        let line = line.as_ref();
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
    pub fn write_array<P: AsRef<str>>(&mut self, a: Vec<P>, lbreak: u32) {
        let mut l = String::new();
        let mut c: u32 = 0;
        for (i, e) in a.iter().enumerate() {
            let e = e.as_ref();
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
