use assert_cmd::Command;

fn get_db_path() -> String {
    let test_db_path = std::env::var("TIMEST_TEST").expect("
        \x1b[0;31m
    The target path for the test database must be specified in the `TIMEST_TEST` 
    environment variable
        \x1b[0m
    ");

    // Make sure the test db path is valid
    let test_db_pathbuf = std::path::PathBuf::from(&test_db_path);
    assert!(test_db_pathbuf.parent().unwrap().is_dir());

    // Clean up db from failed tests
    if test_db_pathbuf.is_file() {
        std::fs::remove_file(test_db_pathbuf).unwrap();
    }

    test_db_path
}

fn new_cmd(test_db_path: &String) -> Command {
    let mut cmd = Command::cargo_bin("timest").unwrap();
    cmd.args(&["--db-path", test_db_path]);
    cmd
}

fn get_output(cmd: &mut Command) -> String {
    String::from_utf8(
        cmd.output()
            .unwrap()
            .stdout
        ).unwrap()
}

#[test]
fn test_1() {
    let test_db_path = get_db_path();

    // No arguments
    let mut cmd = Command::cargo_bin("timest").unwrap();
    cmd.args(&["--db-path", &test_db_path]);
    cmd.assert()
        .append_context("main", "no subcommand")
        .failure();

    // Clock in
    let mut cmd = new_cmd(&test_db_path);
    cmd.args(&["clock", "i", "-d", "2023-05-31", "-t", "8:00"]);
    cmd.assert()
        .success();

    // Read report
    let mut cmd = new_cmd(&test_db_path);
    cmd.args(&["report", "simple", "-d", "2023-05-31"]);
    let output = get_output(&mut cmd);
    let last_line = output.trim()
        .split('\n')
        .last()
        .unwrap();
    assert_eq!(last_line,"|  1  |  08:00:00  |  i  |");

    // Clock out
    let mut cmd = new_cmd(&test_db_path);
    cmd.args(&["clock", "o", "--date", "2023-05-31", "--time", "17:00"]);
    cmd.assert().success();

    // Read fancy report
    let mut cmd = new_cmd(&test_db_path);
    cmd.args(&["report", "-d", "2023-05-31"]);
    let output = get_output(&mut cmd);
    let total_time_worked = output.trim()
        .split_whitespace()
        .last()
        .unwrap();
    assert_eq!(total_time_worked, "9:00:00");

    // Write incomplete clock in
    let mut cmd = new_cmd(&test_db_path);
    cmd.args(&["clock", "o", "-d", "2023-05-31", "-t", "7:00"]);
    cmd.assert().success();

    // Read fancy report with error
    let mut cmd = new_cmd(&test_db_path);
    cmd.args(&["report", "-d", "2023-05-31"]);
    let output = get_output(&mut cmd);
    let output = output.trim()
        .split('\n')
        .collect::<Vec<&str>>();
    let error_message = *output.get(output.len() - 3).unwrap();
    assert_eq!(error_message, "ERROR there are some incomplete intervals");

    // Move incomplete clock in to end of day
    let mut cmd = new_cmd(&test_db_path);
    cmd.args(&["fix", "3", "o", "-d", "2023-05-31", "-t", "19:00"]);
    cmd.assert().success();

    // Check that move worked
    let mut cmd = new_cmd(&test_db_path);
    cmd.args(&["report", "simple", "-d", "2023-05-31"]);
    let output = get_output(&mut cmd);
    let last_line = output.trim()
        .split('\n')
        .last()
        .unwrap();
    assert_eq!(last_line, "|  3  |  19:00:00  |  o  |");

    // Delete clock out at 17:00
    let mut cmd = new_cmd(&test_db_path);
    cmd.args(&["delete", "2"]);
    cmd.assert().success();

    // Make sure that new TOTAL TIME WORKED has updated
    let mut cmd = new_cmd(&test_db_path);
    cmd.args(&["report", "-d", "2023-05-31"]);
    let output = get_output(&mut cmd);
    let total_time_worked = output.trim()
        .split_whitespace()
        .last()
        .unwrap();
    assert_eq!(total_time_worked, "11:00:00");

    std::fs::remove_file(test_db_path).expect("Could not clean up test database. See $TIMEST_TEST");
}