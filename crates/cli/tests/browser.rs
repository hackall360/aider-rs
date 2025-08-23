use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn browser_flag_launches_gui() {
    let mut cmd = Command::cargo_bin("aider-cli").unwrap();
    cmd.env("AIDER_TEST_GUI", "1")
        .args(["--browser", "--yes"]);
    cmd.assert().stdout(contains("launch_gui_called"));
}
