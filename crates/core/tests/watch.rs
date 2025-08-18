use std::fs;

use aider_core::FileWatcher;
use tempfile::tempdir;

#[test]
fn single_event_on_modify() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("file.txt");
    fs::write(&file, b"init").unwrap();

    let watcher = FileWatcher::new(&[dir.path().to_path_buf()]).unwrap();

    fs::write(&file, b"changed").unwrap();
    let paths = watcher.next().expect("event");
    assert_eq!(paths, vec![file]);
}

#[test]
fn symlink_does_not_duplicate() {
    let dir = tempdir().unwrap();
    let file = dir.path().join("real.txt");
    fs::write(&file, b"init").unwrap();
    let link = dir.path().join("link.txt");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&file, &link).unwrap();
    #[cfg(windows)]
    std::os::windows::fs::symlink_file(&file, &link).unwrap();

    let watcher = FileWatcher::new(&[dir.path().to_path_buf()]).unwrap();

    fs::write(&file, b"changed").unwrap();
    let paths = watcher.next().expect("event");
    assert_eq!(paths, vec![file]);
}
