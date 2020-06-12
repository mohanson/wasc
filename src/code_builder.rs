use std::io::Write;

// A C code builder.
pub struct CodeBuilder {
    pub fd: std::fs::File,
    head_whitespace: usize,
}

impl CodeBuilder {
    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let fd = std::fs::File::create(path)?;
        Ok(CodeBuilder { fd, head_whitespace: 0 })
    }

    pub fn write_line(&mut self, line: &str) -> Result<(), Box<dyn std::error::Error>> {
        let a = " ".repeat(self.head_whitespace);
        let b = line;
        let c = "\n";
        self.fd.write_all((a + b + c).as_bytes())?;
        Ok(())
    }

    pub fn intend(&mut self) {
        self.head_whitespace += 2;
    }

    pub fn extend(&mut self) {
        self.head_whitespace -= 2;
    }
}
