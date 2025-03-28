#![allow(unused_variables)]

#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    /// Creates a new empty File with the given name
    ///
    /// # Arguments
    /// * `name` - The name of the file to create
    ///
    /// # Returns
    /// A new `File` instance with the specified name and empty data
    pub fn new(name: &str) -> Self {
        File {
            name: String::from(name),
            data: Vec::new(),
        }
    }

    /// Opens the file for reading
    ///
    /// # Returns
    /// `true` if the file was successfully opened, `false` otherwise
    /// 
    /// # Example
    /// ```rust
    /// let f1 = File::new("f1.txt");
    /// let opened = f1.open();
    /// assert_eq!(opened, true);
    /// ```
    pub fn open(&self) -> bool {
        true
    }

    /// Closes an open file
    ///
    /// # Returns
    /// `true` if the file was successfully closed, `false` otherwise
    pub fn close(&self) -> bool {
        true
    }

    /// Reads the file's contents into the provided buffer
    ///
    /// # Arguments
    /// * `buf` - The buffer to read the file data into
    ///
    /// # Returns
    /// The number of bytes read from the file
    pub fn read(&self, buf: &mut Vec<u8>) -> usize {
        let read_length = self.data.len();

        buf.reserve(read_length);
        buf.append(&mut self.data.clone());

        read_length
    }

    /// Returns the length of the file's data in bytes
    ///
    /// # Returns
    /// The number of bytes in the file's data
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns the name of the file
    ///
    /// # Returns
    /// A string slice containing the file's name
    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Demonstrates basic file operations by creating a file and printing its properties
///
/// This function creates a new empty file and prints its name and length.
///
/// # Example Output
/// ```text
/// File 1 name: f1.txt
/// File 1 length: 0 bytes
/// ```
pub fn try_file_ops() {
    let mut f1 = File::new("f1.txt");

    f1.open();

    println!("File 1 name: {}", f1.name());
    println!("File 1 length: {} bytes", f1.len());

    let input_string = "Hello, world from a file!";
    f1.data.reserve(input_string.len());
    f1.data.append(&mut input_string.as_bytes().to_vec());

    let mut buf = Vec::new();
    let bytes_read = f1.read(&mut buf);
    println!("Bytes read: {}", bytes_read);
    println!("Buffer: {:?}", String::from_utf8_lossy(&buf));

    println!("{:?}", buf == f1.data);

    f1.close();
}
