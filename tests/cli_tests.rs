use assert_cmd::Command;
use predicates::prelude::*;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

mod file_utils;
mod test_context;

use crate::file_utils::rm_file;
use file_utils::create_temp_directory;
use test_context::TearDownTestContext;

const APP_NAME: &str = "actionoscope";
const TEST_FILE_NAME: &str = "test_workflow.yml";

fn create_clean_up_test_workflow_file(file_path: PathBuf) -> impl FnOnce() {
    move || {
        rm_file(&file_path, true).expect("Failed to remove test workflow file");
    }
}

#[test]
fn test_run_single_step() {
    // Given: A temporary directory and a test workflow file are set up
    let tmp_path = create_temp_directory("test_run_single_step")
        .expect("Failed to create temporary directory for test");
    let file_path = tmp_path.join(TEST_FILE_NAME);
    let clean_up_fn = create_clean_up_test_workflow_file(file_path.clone());
    setup_test_workflow(file_path.clone());

    // When: The command to run a single step is executed
    let mut cmd = Command::cargo_bin(APP_NAME).unwrap();
    cmd.arg("run")
        .arg("--workflow-file")
        .arg(&file_path)
        .arg("--job")
        .arg("test_job")
        .arg("--step")
        .arg("step1");

    // Then: The command should succeed and contain the expected output
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Running step name/id 'Step 1'"));

    let _ = TearDownTestContext::new(clean_up_fn);
}

#[test]
fn test_run_single_step_invalid_job() {
    // Given: A temporary directory and a test workflow file are set up
    let tmp_path = create_temp_directory("test_run_single_step_invalid_job")
        .expect("Failed to create temporary directory for test");
    let file_path = tmp_path.join(TEST_FILE_NAME);
    let clean_up_fn = create_clean_up_test_workflow_file(file_path.clone());
    setup_test_workflow(file_path.clone());

    // When: The command to run a single step with an invalid job is executed
    let mut cmd = Command::cargo_bin(APP_NAME).unwrap();
    cmd.arg("run")
        .arg("--workflow-file")
        .arg(&file_path)
        .arg("--job")
        .arg("invalid_job")
        .arg("--step")
        .arg("step1");

    // Then: The command should fail
    cmd.assert().failure();

    let _ = TearDownTestContext::new(clean_up_fn);
}

#[test]
fn test_run_single_step_invalid_step() {
    // Given: A temporary directory and a test workflow file are set up
    let tmp_path = create_temp_directory("test_run_single_step_invalid_step")
        .expect("Failed to create temporary directory for test");
    let file_path = tmp_path.join(TEST_FILE_NAME);
    let clean_up_fn = create_clean_up_test_workflow_file(file_path.clone());
    setup_test_workflow(file_path.clone());

    // When: The command to run an invalid step is executed
    let mut cmd = Command::cargo_bin(APP_NAME).unwrap();
    cmd.arg("run")
        .arg("--workflow-file")
        .arg(&file_path)
        .arg("--job")
        .arg("test_job")
        .arg("--step")
        .arg("invalid_step");

    // Then: The command should fail
    cmd.assert().failure();

    let _ = TearDownTestContext::new(clean_up_fn);
}

#[test]
fn test_run_all_steps_since_invalid_step() {
    // Given: A temporary directory and a test workflow file are set up
    let tmp_path = create_temp_directory("test_run_all_steps_since_invalid_step")
        .expect("Failed to create temporary directory for test");
    let file_path = tmp_path.join(TEST_FILE_NAME);
    let clean_up_fn = create_clean_up_test_workflow_file(file_path.clone());
    setup_test_workflow(file_path.clone());

    // When: The command to run all steps since an invalid step is executed
    let mut cmd = Command::cargo_bin(APP_NAME).unwrap();
    cmd.arg("run")
        .arg("--workflow-file")
        .arg(&file_path)
        .arg("--job")
        .arg("test_job")
        .arg("--from-step")
        .arg("invalid_step");

    // Then: The command should fail
    cmd.assert().failure();

    let _ = TearDownTestContext::new(clean_up_fn);
}

#[test]
fn test_run_all_steps_since() {
    // Given: A temporary directory and a test workflow file are set up
    let tmp_path = create_temp_directory("test_run_all_steps_since")
        .expect("Failed to create temporary directory for test");
    let file_path = tmp_path.join(TEST_FILE_NAME);
    let clean_up_fn = create_clean_up_test_workflow_file(file_path.clone());
    setup_test_workflow(file_path.clone());

    // When: The command to run all steps since a specific step is executed
    let mut cmd = Command::cargo_bin(APP_NAME).unwrap();
    cmd.arg("run")
        .arg("--workflow-file")
        .arg(&file_path)
        .arg("--job")
        .arg("test_job")
        .arg("--from-step")
        .arg("step2");

    // Then: The command should succeed and contain the expected output
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Running step name/id 'Step 2'"))
        .stdout(predicate::str::contains("Running step name/id 'Step 3'"));

    let _ = TearDownTestContext::new(clean_up_fn);
}

#[test]
fn test_run_all_steps_except_skip_steps() {
    // Given: A temporary directory and a test workflow file are set up
    let tmp_path = create_temp_directory("test_run_all_steps_except_skip_steps")
        .expect("Failed to create temporary directory for test");
    let file_path = tmp_path.join(TEST_FILE_NAME);
    let clean_up_fn = create_clean_up_test_workflow_file(file_path.clone());
    setup_test_workflow(file_path.clone());

    // When: The command to run all steps except skipped steps is executed
    let mut cmd = Command::cargo_bin(APP_NAME).unwrap();
    cmd.arg("run")
        .arg("--workflow-file")
        .arg(&file_path)
        .arg("--job")
        .arg("test_job")
        .arg("--skip-step")
        .arg("step2")
        .arg("--skip-step")
        .arg("step4");

    // Then: The command should succeed and contain the expected output
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Running step name/id 'Step 1'"))
        .stdout(predicate::str::contains("Running step name/id 'Step 3'"));

    let _ = TearDownTestContext::new(clean_up_fn);
}

fn setup_test_workflow(file_path: PathBuf) {
    let workflow_content: &str = r#"
    name: Test Workflow
    on:
      push:
        branches:
          - main
    jobs:
      test_job:
        runs-on: ubuntu-latest
        steps:
          - name: Step 1
            id: step1
            run: echo "Step 1"
          - name: Step 2
            id: step2
            run: echo "Step 2"
          - name: Step 3
            id: step3
            run: echo "Step 3"
          - name: Step 4
            id: step4
            run: echo "Step 4"
    "#;
    let mut file = File::create(&file_path).expect("Failed to create file");
    writeln!(file, "{}", workflow_content).expect("Failed to write to file");
}
