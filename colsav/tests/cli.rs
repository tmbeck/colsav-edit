use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

use colonization_sav::SaveFile;

fn fixture_save_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../saves/COLONY01.SAV")
}

fn unique_temp_path(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after unix epoch")
        .as_nanos();
    env::temp_dir().join(format!("{prefix}_{}_{}.sav", std::process::id(), nanos))
}

fn run_colsav(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_colsav"))
        .args(args)
        .output()
        .expect("failed to run colsav")
}

fn stdout_string(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr_string(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn test_info_outputs_expected_header_fields() {
    let save_path = fixture_save_path();
    let save_path_string = save_path.to_string_lossy();

    let output = run_colsav(&["info", "-f", &save_path_string]);
    assert!(
        output.status.success(),
        "info should succeed, stderr: {}",
        stderr_string(&output)
    );

    let stdout = stdout_string(&output);
    assert!(!stdout.trim().is_empty(), "info stdout should be non-empty");
    assert!(stdout.contains("Year:"), "info output should contain Year:");
    assert!(stdout.contains("Turn:"), "info output should contain Turn:");
    assert!(stdout.contains("Map:"), "info output should contain Map:");
}

#[test]
fn test_dump_units_succeeds_and_contains_first_unit_marker() {
    let save_path = fixture_save_path();
    let save_path_string = save_path.to_string_lossy();

    let output = run_colsav(&["dump-units", "-f", &save_path_string]);
    assert!(
        output.status.success(),
        "dump-units should succeed, stderr: {}",
        stderr_string(&output)
    );

    let stdout = stdout_string(&output);
    assert!(
        !stdout.trim().is_empty(),
        "dump-units stdout should be non-empty"
    );
    assert!(
        stdout.contains("Unit 1:"),
        "dump-units output should contain first unit marker"
    );
}

#[test]
fn test_dump_colonies_succeeds_and_contains_colony_name() {
    let save_path = fixture_save_path();
    let save_path_string = save_path.to_string_lossy();
    let save = SaveFile::from_path(&save_path).expect("fixture save should parse");
    let first_colony_name = save
        .colonies
        .first()
        .expect("fixture save should contain at least one colony")
        .name();

    let output = run_colsav(&["dump-colonies", "-f", &save_path_string]);
    assert!(
        output.status.success(),
        "dump-colonies should succeed, stderr: {}",
        stderr_string(&output)
    );

    let stdout = stdout_string(&output);
    assert!(
        !stdout.trim().is_empty(),
        "dump-colonies stdout should be non-empty"
    );
    assert!(
        stdout.contains(&first_colony_name),
        "dump-colonies output should include at least one colony name"
    );
}

#[test]
fn test_dump_nations_succeeds_and_contains_england() {
    let save_path = fixture_save_path();
    let save_path_string = save_path.to_string_lossy();

    let output = run_colsav(&["dump-nations", "-f", &save_path_string]);
    assert!(
        output.status.success(),
        "dump-nations should succeed, stderr: {}",
        stderr_string(&output)
    );

    let stdout = stdout_string(&output);
    assert!(
        !stdout.trim().is_empty(),
        "dump-nations stdout should be non-empty"
    );
    assert!(
        stdout.contains("Nation: England"),
        "dump-nations output should contain Nation: England"
    );
}

#[test]
fn test_dump_map_succeeds_and_matches_map_height() {
    let save_path = fixture_save_path();
    let save_path_string = save_path.to_string_lossy();
    let save = SaveFile::from_path(&save_path).expect("fixture save should parse");

    let output = run_colsav(&["dump-map", "-f", &save_path_string]);
    assert!(
        output.status.success(),
        "dump-map should succeed, stderr: {}",
        stderr_string(&output)
    );

    let stdout = stdout_string(&output);
    assert!(!stdout.is_empty(), "dump-map stdout should be non-empty");

    let line_count = stdout.lines().count();
    assert_eq!(
        line_count, save.tile_map.rows,
        "dump-map should print one line per map row"
    );
}

#[test]
fn test_edit_gold_updates_output_file() {
    let save_path = fixture_save_path();
    let output_path = unique_temp_path("colsav_edit_gold");
    let output_path_string = output_path.to_string_lossy();
    let save_path_string = save_path.to_string_lossy();

    let output = run_colsav(&[
        "edit",
        "-f",
        &save_path_string,
        "-o",
        &output_path_string,
        "-p",
        "0",
        "-g",
        "500000",
    ]);
    assert!(
        output.status.success(),
        "edit gold should succeed, stderr: {}",
        stderr_string(&output)
    );

    let edited = SaveFile::from_path(&output_path).expect("edited save should parse");
    assert_eq!(edited.nations[0].gold, 500000, "gold should be updated");

    let _ = fs::remove_file(&output_path);
}

#[test]
fn test_edit_tax_updates_output_file() {
    let save_path = fixture_save_path();
    let output_path = unique_temp_path("colsav_edit_tax");
    let output_path_string = output_path.to_string_lossy();
    let save_path_string = save_path.to_string_lossy();

    let output = run_colsav(&[
        "edit",
        "-f",
        &save_path_string,
        "-o",
        &output_path_string,
        "-p",
        "0",
        "-t",
        "10",
    ]);
    assert!(
        output.status.success(),
        "edit tax should succeed, stderr: {}",
        stderr_string(&output)
    );

    let edited = SaveFile::from_path(&output_path).expect("edited save should parse");
    assert_eq!(edited.nations[0].tax_rate, 10, "tax should be updated");

    let _ = fs::remove_file(&output_path);
}

#[test]
fn test_edit_with_missing_file_fails() {
    let missing_path = unique_temp_path("colsav_missing_input");
    let output_path = unique_temp_path("colsav_missing_output");
    let missing_path_string = missing_path.to_string_lossy();
    let output_path_string = output_path.to_string_lossy();

    let output = run_colsav(&[
        "edit",
        "-f",
        &missing_path_string,
        "-o",
        &output_path_string,
        "-p",
        "0",
        "-g",
        "123",
    ]);
    assert!(
        !output.status.success(),
        "edit with missing file should fail"
    );
}

#[test]
fn test_edit_with_invalid_power_index_fails() {
    let save_path = fixture_save_path();
    let output_path = unique_temp_path("colsav_invalid_power");
    let save_path_string = save_path.to_string_lossy();
    let output_path_string = output_path.to_string_lossy();

    let output = run_colsav(&[
        "edit",
        "-f",
        &save_path_string,
        "-o",
        &output_path_string,
        "-p",
        "4",
        "-g",
        "123",
    ]);
    assert!(
        !output.status.success(),
        "edit with power index outside 0..=3 should fail"
    );
}

#[test]
fn test_edit_without_gold_or_tax_is_no_op() {
    let save_path = fixture_save_path();
    let output_path = unique_temp_path("colsav_edit_noop");
    let output_path_string = output_path.to_string_lossy();
    let save_path_string = save_path.to_string_lossy();

    let output = run_colsav(&[
        "edit",
        "-f",
        &save_path_string,
        "-o",
        &output_path_string,
        "-p",
        "0",
    ]);
    assert!(
        output.status.success(),
        "edit without -g/-t should succeed, stderr: {}",
        stderr_string(&output)
    );

    let input_bytes = fs::read(&save_path).expect("fixture save should be readable");
    let output_bytes = fs::read(&output_path).expect("no-op edited save should be readable");
    assert_eq!(
        output_bytes, input_bytes,
        "no-op edit should preserve bytes exactly"
    );

    let _ = fs::remove_file(&output_path);
}
