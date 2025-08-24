/// Parse fenced unified diff blocks from text and return file edits.
///
/// Each edit is returned as a tuple of the file path and the lines in the hunk
/// (including context and +/- changes, but excluding the `@@` header lines).
pub fn find_diffs(content: &str) -> Vec<(String, Vec<String>)> {
    let mut text = content.to_string();
    if !text.ends_with('\n') {
        text.push('\n');
    }
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let mut i = 0;
    let mut edits = Vec::new();

    while i < lines.len() {
        if lines[i].starts_with("```diff") {
            i += 1; // move past ```diff line
            let mut block = Vec::new();
            while i < lines.len() && !lines[i].starts_with("```") {
                block.push(lines[i].to_string());
                i += 1;
            }
            // skip closing fence if present
            if i < lines.len() {
                i += 1;
            }

            // determine file name from header if present
            let mut fname = String::new();
            let mut start = 0usize;
            if block.len() >= 2 && block[0].starts_with("--- ") && block[1].starts_with("+++ ") {
                let a_fname = block[0][4..].trim();
                let b_fname = block[1][4..].trim();
                if (a_fname.starts_with("a/") || a_fname == "/dev/null") && b_fname.starts_with("b/") {
                    fname = b_fname[2..].to_string();
                } else {
                    fname = b_fname.to_string();
                }
                start = 2;
            }

            // scan for hunks inside the block, accounting for multiple diffs
            let mut j = start;
            let mut current_fname = fname.clone();
            while j < block.len() {
                if block[j].starts_with("--- ") && j + 1 < block.len() && block[j + 1].starts_with("+++ ") {
                    let a_fname = block[j][4..].trim();
                    let b_fname = block[j + 1][4..].trim();
                    if (a_fname.starts_with("a/") || a_fname == "/dev/null") && b_fname.starts_with("b/") {
                        current_fname = b_fname[2..].to_string();
                    } else {
                        current_fname = b_fname.to_string();
                    }
                    j += 2;
                    continue;
                }
                if block[j].starts_with("@@") {
                    j += 1;
                    let mut hunk = Vec::new();
                    while j < block.len()
                        && !block[j].starts_with("@@")
                        && !(block[j].starts_with("--- ") && j + 1 < block.len() && block[j + 1].starts_with("+++ "))
                        && !(block[j].trim().is_empty()
                            && j + 2 < block.len()
                            && block[j + 1].starts_with("--- ")
                            && block[j + 2].starts_with("+++ "))
                    {
                        hunk.push(block[j].clone());
                        j += 1;
                    }
                    if !hunk.is_empty() {
                        edits.push((current_fname.clone(), hunk));
                    }
                    continue;
                }
                j += 1;
            }
        } else {
            i += 1;
        }
    }

    edits
}
