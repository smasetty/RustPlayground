/// Searches for a pattern in a text and returns a string of the lines that match
///
/// # Arguments
/// * `pattern` - The pattern to search for
/// * `text` - The text to search in
///
/// # Returns
/// A String containing all matching lines
pub fn grep_internal(pattern: &str, text: &str) -> String {
    let mut result = String::new();

    for line in text.lines() {
        if line.contains(pattern) {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

/// Searches for a pattern in text and returns context around matches
///
/// # Arguments
/// * `pattern` - The pattern to search for
/// * `text` - The text to search in
///
/// # Returns
/// A vector of vectors containing line numbers and content around matches
pub fn grep_internal_2(pattern: &str, text: &str) -> Vec<Vec<(usize, String)>> {
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

/// Demonstrates the grep functionality with a simple example
pub fn try_grep() {
    let text = "I'm good. I'm great. I'm doing good.";
    let pattern = "good";
    let result: String = grep_internal(pattern, text);
    println!("{} searching for {}", "try_grep", pattern);
    println!("result: {}", result);
}

/// Demonstrates the context-aware grep functionality
pub fn try_grep_2() {
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