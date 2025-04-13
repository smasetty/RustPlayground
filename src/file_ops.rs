#[derive(Debug, PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
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
            state: FileState::Closed,
        }
    }

    pub fn new_with_data(
        name: &str,
        data: &Vec<u8>
    ) -> Self {
        let mut file = File::new(name);
        file.data = data.clone();
        file
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
    pub fn open(&mut self) -> Result<(), String> {
        self.state = FileState::Open;
        Ok(())
    }

    /// Closes an open file
    ///
    /// # Returns
    /// `true` if the file was successfully closed, `false` otherwise
    pub fn close(&mut self) -> Result<(), String> {
        self.state = FileState::Closed;
        Ok(())
    }

    /// Test function to open, read, and close a file
    ///
    /// # Returns
    /// `Ok(())` if the file was successfully opened, read, and closed
    /// `Err(String)` if any operation fails
    /// 
    /// # Example
    /// ```rust
    /// let mut f1 = File::new("f1.txt");
    /// let result = f1.testFn();
    /// assert_eq!(result, Ok(()));
    /// ```
    pub fn test_file_open_close(&mut self) -> Result<(), String> {
        self.open()?;
        self.read(&mut Vec::new())?;
        self.close()?;
        Ok(())
    }

    /// Reads the file's contents into the provided buffer
    ///
    /// # Arguments
    /// * `buf` - The buffer to read the file data into
    ///
    /// # Returns
    /// The number of bytes read from the file
    pub fn read(&self, buf: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File is not open"));
        }

        let read_length = self.data.len();
        if read_length <= 0 {
            return Err(String::from("File is empty"));
        }

        buf.reserve(read_length);
        buf.extend_from_slice(&self.data);

        Ok(read_length)
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
    let data = vec![1, 2, 3, 4, 5];
    let mut f1 = File::new_with_data("f1.txt", &data);

    if let Err(e) = f1.test_file_open_close() {
        println!("Error with test_file_open_close: {}", e);
        return;
    }

    if let Err(e) = f1.open() {
        println!("Error opening file: {}", e);
        return;
    }

    if f1.state != FileState::Open {
        println!("File is not open");
        return;
    }

    println!("File 1 name: {}", f1.name());
    println!("File 1 length: {} bytes", f1.len());

    // let input_string = "Hello, world from a file!";
    // f1.data.reserve(input_string.len());
    // f1.data.append(&mut input_string.as_bytes().to_vec());

    let mut buf = Vec::new();
    let bytes_read = f1.read(&mut buf);
    println!("Bytes read: {}", bytes_read.unwrap());
    println!("Buffer: {:?}", String::from_utf8_lossy(&buf));

    println!("{:?}", buf == f1.data);

    if let Err(e) = f1.close() {
        println!("Error closing file: {}", e);
    }
}