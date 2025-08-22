use diffy::create_patch;
use similar::{ChangeTag, TextDiff};

/// Create a progress bar string for the given percentage.
fn create_progress_bar(percentage: usize) -> String {
    const TOTAL: usize = 30;
    let filled = TOTAL * percentage / 100;
    let empty = TOTAL - filled;
    let block = "█".repeat(filled);
    let empty = "░".repeat(empty);
    format!("{block}{empty}")
}

fn assert_newlines(lines: &[String]) {
    if lines.is_empty() {
        return;
    }
    for line in &lines[..lines.len() - 1] {
        assert!(line.ends_with('\n'), "missing newline");
    }
}

fn find_last_non_deleted(lines_orig: &[String], lines_updated: &[String]) -> Option<usize> {
    let orig = lines_orig.join("");
    let updated = lines_updated.join("");
    let diff = TextDiff::from_lines(&orig, &updated);
    let mut num_orig = 0usize;
    let mut last = None;
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Equal => {
                num_orig += 1;
                last = Some(num_orig);
            }
            ChangeTag::Delete => {
                num_orig += 1;
            }
            ChangeTag::Insert => {}
        }
    }
    last
}

/// Generate a unified diff for a partial update.
///
/// This mirrors the behavior of the Python implementation in `aider/diffs.py`.
/// When `final_update` is false, deleted lines beyond the last updated line are
/// ignored and a progress bar is inserted to indicate the amount of work
/// completed.
pub fn diff_partial_update(
    lines_orig: &[String],
    lines_updated: &[String],
    final_update: bool,
    fname: Option<&str>,
) -> String {
    assert_newlines(lines_orig);
    let num_orig_lines = lines_orig.len();

    let last_non_deleted = if final_update {
        num_orig_lines
    } else {
        match find_last_non_deleted(lines_orig, lines_updated) {
            Some(n) => n,
            None => return String::new(),
        }
    };

    let pct = if num_orig_lines > 0 {
        last_non_deleted * 100 / num_orig_lines
    } else {
        50
    };
    let bar = create_progress_bar(pct);
    let bar_line = format!(" {last_non_deleted:3} / {num_orig_lines:3} lines [{bar}] {pct:3}%\n");

    let orig_prefix = lines_orig[..last_non_deleted].join("");
    let mut updated_lines = lines_updated.to_vec();
    if !final_update {
        if !updated_lines.is_empty() {
            updated_lines.pop();
        }
        updated_lines.push(bar_line.clone());
    }
    let updated_text = updated_lines.join("");

    let patch = create_patch(&orig_prefix, &updated_text);
    let mut diff = patch.to_string();
    if !diff.ends_with('\n') {
        diff.push('\n');
    }

    let mut ticks = 3usize;
    while diff.contains(&"`".repeat(ticks)) && ticks < 10 {
        ticks += 1;
    }
    let fence = "`".repeat(ticks);
    let mut show = format!("{fence}diff\n");
    if let Some(name) = fname {
        show.push_str(&format!("--- {name} original\n+++ {name} updated\n"));
    }
    show.push_str(&diff);
    show.push_str(&format!("{fence}\n\n"));
    show
}

/// Generate a unified diff for two complete texts.
pub fn unified_diff(orig: &str, updated: &str, fname: Option<&str>) -> String {
    let patch = create_patch(orig, updated);
    let mut diff = patch.to_string();
    if !diff.ends_with('\n') {
        diff.push('\n');
    }
    let mut ticks = 3usize;
    while diff.contains(&"`".repeat(ticks)) && ticks < 10 {
        ticks += 1;
    }
    let fence = "`".repeat(ticks);
    let mut out = format!("{fence}diff\n");
    if let Some(name) = fname {
        out.push_str(&format!("--- {name} original\n+++ {name} updated\n"));
    }
    out.push_str(&diff);
    out.push_str(&format!("{fence}\n\n"));
    out
}
