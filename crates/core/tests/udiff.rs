use aider_core::find_diffs;

#[test]
fn find_diffs_single_hunk() {
    let content = r#"Some text...

```diff
--- file.txt
+++ file.txt
@@ ... @@
-Original
+Modified
```
"#;
    let edits = find_diffs(content);
    assert_eq!(edits.len(), 1);
    let edit = &edits[0];
    assert_eq!(edit.0, "file.txt");
    assert_eq!(edit.1, vec!["-Original\n".to_string(), "+Modified\n".to_string()]);
}

#[test]
fn find_diffs_dev_null() {
    let content = r#"Some text...

```diff
--- /dev/null
+++ file.txt
@@ ... @@
-Original
+Modified
```
"#;
    let edits = find_diffs(content);
    assert_eq!(edits.len(), 1);
    let edit = &edits[0];
    assert_eq!(edit.0, "file.txt");
    assert_eq!(edit.1, vec!["-Original\n".to_string(), "+Modified\n".to_string()]);
}

#[test]
fn find_diffs_dirname_with_spaces() {
    let content = r#"Some text...

```diff
--- dir name with spaces/file.txt
+++ dir name with spaces/file.txt
@@ ... @@
-Original
+Modified
```
"#;
    let edits = find_diffs(content);
    assert_eq!(edits.len(), 1);
    let edit = &edits[0];
    assert_eq!(edit.0, "dir name with spaces/file.txt");
    assert_eq!(edit.1, vec!["-Original\n".to_string(), "+Modified\n".to_string()]);
}

#[test]
fn find_multi_diffs() {
    let content = r#"To implement the `--check-update` option, I will make the following changes:

1. Add the `--check-update` argument to the argument parser in `aider/main.py`.
2. Modify the `check_version` function in `aider/versioncheck.py` to return a boolean indicating whether an update is available.
3. Use the returned value from `check_version` in `aider/main.py` to set the exit status code when `--check-update` is used.

Here are the diffs for those changes:

```diff
--- aider/versioncheck.py
+++ aider/versioncheck.py
@@ ... @@
     except Exception as err:
         print_cmd(f"Error checking pypi for new version: {err}")
+        return False

--- aider/main.py
+++ aider/main.py
@@ ... @@
     other_group.add_argument(
         "--version",
         action="version",
         version=f"%(prog)s {__version__}",
         help="Show the version number and exit",
     )
+    other_group.add_argument(
+        "--check-update",
+        action="store_true",
+        help="Check for updates and return status in the exit code",
+        default=False,
+    )
     other_group.add_argument(
         "--apply",
         metavar="FILE",
```

These changes will add the `--check-update` option to the command-line interface and use the `check_version` function to determine if an update is available, exiting with status code `0` if no update is available and `1` if an update is available.
"#;
    let edits = find_diffs(content);
    assert_eq!(edits.len(), 2);
    assert_eq!(edits[0].1.len(), 3);
}
