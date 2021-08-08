use assert_cmd::Command;

#[test]
fn handle_unrecognized_cmd() {
    let mut cmd = Command::cargo_bin("cb").unwrap();

    let assert = cmd.args(&["foo"]).assert();
    assert.stderr("error: Found argument \'foo\' which wasn\'t expected, or isn\'t valid in this context\n\nIf you tried to supply `foo` as a PATTERN use `-- foo`\n\nUSAGE:\n    cb <SUBCOMMAND>\n\nFor more information try --help\n");
}
