/// Main entry point for the application
fn main() {
    println!("Hello, world!");
    process_penguin_data();
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