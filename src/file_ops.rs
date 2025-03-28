#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    pub fn new(name: &str) -> Self {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

pub fn try_file_ops() {
    let f1 = File::new("f1.txt");

    println!("File 1 name: {}", f1.name());
    println!("File 1 length: {} bytes", f1.len());
}
