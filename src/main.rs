use std::time::{Duration, Instant};
fn main() {
    println!("Hello, world!");
    process_penguin_data();
    try_simple_math();

    try_while_loop();
}

/// Counts the number of iterations possible within a one-second duration
///
/// This function demonstrates the use of Rust's time measurement capabilities by:
/// 1. Starting a timer
/// 2. Incrementing a counter in a tight loop for one second
/// 3. Printing the final count, showing how many iterations were completed
///
/// The count gives a rough indication of the system's processing speed, though
/// it should not be used as a formal benchmark due to various factors that
/// can affect the results.
///
/// # Example Output
/// ```text
/// count: 234567890  // Actual number will vary by system
/// ```
fn try_while_loop() {
    let mut count = 0;
    let duration = Duration::new(1, 0);  // One second duration
    let start = Instant::now();

    while (Instant::now() - start) < duration {
        count += 1;
    }

    println!("count: {}", count);
}

/// Adds two 32-bit integers and returns their sum
///
/// # Arguments
/// * `a` - The first integer operand
/// * `b` - The second integer operand
///
/// # Returns
/// The sum of `a` and `b` as an i32
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Demonstrates basic arithmetic operations using the add function
///
/// This function creates two integer variables, adds them using the `add` function,
/// and prints the result to standard output.
fn try_simple_math() {
    println!("\n{}", "try_simple_math");
    let a = 1_i32;
    let b: i32 = 2;
    let c = 30_i32;
    let d = 30_i32;

    let e:i32 = add (add(a, b), add(c, d));
    println!("sum: {}", e);
}

/// Processes penguin data from a CSV-like string format and prints valid records
///
/// This function takes a hardcoded string containing penguin measurements including:
/// - Common name
/// - Length (in cm)
/// - Wingspan (in cm)
/// - Mass (in kg)
///
/// The function parses each line, validates the length field, and prints valid records.
/// Invalid length values are logged as errors.
///
/// # Debug Mode
/// When compiled in debug mode, additional debug information is printed showing
/// the raw fields for each record.
fn process_penguin_data() {
    let penguin_data = "\
    common name,length,wingspan,mass
    little penguin,33,71,1.6
    yellow-eyed penguin,65,130,3.5
    Fiordland penguin,75,140,3.2
    emperor penguin,115,200,23
    adelie penguin,70,90,4.0
    gentoo penguin,75,100,5.5
    chinstrap penguin,72,95,3.8
    king penguin,95,180,14.5
    macaroni penguin,70,95,4.5
    rockhopper penguin,55,80,2.7
    invalid_penguin,abc,def,xyz
    ";

    let records = penguin_data.lines();

    // Find max widths for each column
    let mut max_widths = (0, 0, 0, 0);
    for (i, record) in records.clone().enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }
        let fields: Vec<_> = record.split(',').map(|field| field.trim()).collect();
        max_widths.0 = max_widths.0.max(fields[0].len());
        max_widths.1 = max_widths.1.max(fields[1].len());
        max_widths.2 = max_widths.2.max(fields[2].len());
        max_widths.3 = max_widths.3.max(fields[3].len());
    }

    // Print header
    println!("{:<width0$}  {:<width1$}  {:<width2$}  {:<width3$}",
        "Name", "Length", "Wingspan", "Mass",
        width0 = max_widths.0,
        width1 = max_widths.1,
        width2 = max_widths.2,
        width3 = max_widths.3
    );

    // Print separator line
    println!("{}", "-".repeat(max_widths.0 + max_widths.1 + max_widths.2 + max_widths.3 + 15));

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record
                .split(',')
                .map(|field| field.trim())
                .collect();

        if cfg!(debug_assertions) {
            eprintln!("debug: {:?} -> {:?}", i, fields);
        }

        let name = fields[0];
        let length = fields[1];
        let wingspan = fields[2];
        let mass = fields[3];

        if let Ok(length) = length.parse::<f32>() {
            println!("{:<width0$}  {:<width1$}  {:<width2$}  {:<width3$}",
                name, length, wingspan, mass,
                width0 = max_widths.0,
                width1 = max_widths.1,
                width2 = max_widths.2,
                width3 = max_widths.3
            );
        } else {
            eprintln!("Invalid length value: {}", length);
        }
    }
}