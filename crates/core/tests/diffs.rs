use aider_core::unified_diff;

#[test]
fn unified_diff_shows_changes() {
    let orig = "old\n";
    let updated = "new\n";
    let diff = unified_diff(orig, updated, Some("file.txt"));
    assert!(diff.contains("-old"));
    assert!(diff.contains("+new"));
    assert!(diff.contains("file.txt"));
}
