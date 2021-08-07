use assert_cmd::Command;

#[test]
fn get_value_from_clipboard() {
    let mut cmd = Command::cargo_bin("cb").unwrap();

    let assert = cmd
        .args(&["get"])
        .assert();
    assert.stdout("clipboard contents: \"\"\n");
}

#[test]
fn set_value_to_clipboard() {
    let mut cmd = Command::cargo_bin("cb").unwrap();

    let assert = cmd
        .args(&["set"])
        .assert();
    assert.stdout("value set to clipboard!\n");
}

#[test]
fn clear_clipboard() {
    let mut cmd = Command::cargo_bin("cb").unwrap();

    let assert = cmd
        .args(&["clear"])
        .assert();
    assert.stdout("clipboard cleared!\n");
}

#[test]
fn handle_unrecognized_cmd() {
    let mut cmd = Command::cargo_bin("cb").unwrap();

    let assert = cmd
        .args(&["foo"])
        .assert();
    assert.stderr("error: Found argument \'foo\' which wasn\'t expected, or isn\'t valid in this context\n\nIf you tried to supply `foo` as a PATTERN use `-- foo`\n\nUSAGE:\n    cb <SUBCOMMAND>\n\nFor more information try --help\n");
}
