use std::time::{Duration, Instant};
fn main() {
    println!("Hello, world!");
    process_penguin_data();

    try_simple_math();
    try_while_loop();

    try_is_even(2);
    try_is_even(3);
    try_match();
    try_grep();
    try_grep_2();
}

/// Demonstrates checking if a number is even or odd
///
/// This function creates a number, checks if it's even using the `is_even` function,
/// and prints a description of the number to standard output.
///
/// # Example Output
/// ```text
/// 2 is even
/// ```
fn try_is_even(a: i32) {

    let description = match is_even(a) {
        true => "even",
        false => "odd",
    };

    println!("{} is {}", a, description);
}

fn try_match() {
    let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &haystack {
        let result = match *item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        println!("{}: {}", *item, result);
    }
}

fn try_grep() {
    let text = "I'm good. I'm great. I'm doing good.";
    let pattern = "good";
    // Search for the pattern in the text and print matching lines
    let result: String = grep_internal(pattern, text);
    println!("{} searching for {}", "try_grep", pattern);
    println!("result: {}", result);
}

/// Searches for a pattern in a text and returns a string of the lines that match
///
/// # Arguments
/// * `pattern` - The pattern to search for
/// * `text` - The text to search in
///
/// # Returns
fn grep_internal(pattern: &str, text: &str) -> String {

    let mut result = String::new();

    for line in text.lines() {
        if line.contains(pattern) {
            result.push_str(line);
            result.push('\n');
        }
    }

    return result;
}

fn try_grep_2() {
    let text = "\
    The autumn leaves dance gently,
    carried by the evening breeze.
    A distant train whistles,
    echoing through empty streets.
    Time flows like a river,
    never pausing, never waiting.
    The city sleeps beneath stars,
    while dreams take flight silently.
    Stories unfold in shadows,
    leaving footprints in memory.
    ";
    let pattern = "river";
    let result = grep_internal_2(pattern, text);
    println!("{} searching for {}", "try_grep_2", pattern);
    println!("result: {:?}", result);
}

fn grep_internal_2(pattern: &str, text: &str) -> Vec<Vec<(usize, String)>> {
    let ctx_size = 2;
    let mut tags: Vec<usize> = vec![];
    let mut ctx: Vec<Vec<(usize, String)>> = vec![];

    for (i, line) in text.lines().enumerate() {
        if line.contains(pattern) {
             tags.push(i);
             let v = Vec::with_capacity(ctx_size * 2 + 1);
             ctx.push(v);
        }
    }

    if tags.is_empty() {
        return ctx;
    }

    for (i, line) in text.lines().enumerate() {
        for (j, tag) in tags.iter().enumerate() {
            let lower = tag.saturating_sub(ctx_size);
            let upper = tag + ctx_size;
            if i >= lower && i <= upper {
                let line_as_string = line.to_string();
                let local_ctx = (i, line_as_string);
                ctx[j].push(local_ctx);
            }
        }
    }

    ctx
}

/// Determines if a number is even
///
/// # Arguments
/// * `n` - The integer to check
///
/// # Returns
/// `true` if the number is even, `false` if odd
fn is_even(n: i32) -> bool {
    return n % 2 == 0;
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