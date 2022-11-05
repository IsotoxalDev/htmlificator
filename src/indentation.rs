// Code Snippet from https://github.com/mgeisler/textwrap/

pub fn indent(s: &str, prefix: &str) -> String {
    let mut result = String::with_capacity(2 * s.len());
    let trimmed_prefix = prefix.trim_end();
    for (idx, line) in s.split_terminator('\n').enumerate() {
        if idx > 0 {
            result.push('\n');
        }
        if line.trim().is_empty() {
            result.push_str(trimmed_prefix);
        } else {
            result.push_str(prefix);
        }
        result.push_str(line);
    }
    if s.ends_with('\n') {
        // split_terminator will have eaten the final '\n'.
        result.push('\n');
    }
    result
}