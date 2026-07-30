//! Splits text on blank-line boundaries, preserving separators.

/// Splits `text` on blank-line boundaries (one or more blank lines),
/// keeping the separating newlines attached so spacing is preserved.
pub fn split_paragraphs(text: &str) -> Vec<String> {
    let mut units: Vec<String> = Vec::new();
    let mut cur = String::new();

    // Mirror splitting text into lines that each keep their trailing "\n"
    // attached, with a final fragment (possibly empty) with no trailing
    // newline also included — matching Go's strings.SplitAfter(text, "\n").
    let mut lines: Vec<String> = Vec::new();
    let mut line_start = 0usize;
    let bytes_indices: Vec<(usize, char)> = text.char_indices().collect();
    for &(byte_idx, c) in &bytes_indices {
        if c == '\n' {
            let end = byte_idx + c.len_utf8();
            lines.push(text[line_start..end].to_string());
            line_start = end;
        }
    }
    lines.push(text[line_start..].to_string());

    for line in lines {
        let is_blank = line.trim().is_empty();
        cur.push_str(&line);
        if is_blank {
            units.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        units.push(cur);
    }
    units
}
